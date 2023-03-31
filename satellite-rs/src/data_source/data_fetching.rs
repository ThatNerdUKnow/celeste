use std::{
    collections::{BTreeSet, HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use log::{debug, error, info, trace};
use reqwest_wasm::{Client, Request, Url};
use result_inspect::ResultInspect;
use sgp4::Elements;
use wasm_bindgen::prelude::*;

use error_stack::{IntoReport, Report, ResultExt};
use web_sys::window;

use crate::{
    bindings::entity_collection::EntityCollection,
    data::{group::Group, static_data::CATEGORIES},
    data_source::{data_fetching::adapter::ElementsAdapter, SatelliteDataSource},
    error::{adapter::ErrorStackAdapter, Error, WrapSgp4Error},
    satellite::{error::SatelliteError, Satellite},
};

pub mod adapter;

static BASE_URL: &str = "https://celestrak.org/NORAD/elements/gp.php";
static LINES_PER_TLE: usize = 3;

#[wasm_bindgen]
impl SatelliteDataSource {
    #[wasm_bindgen]
    pub async fn load_data(&mut self) -> Result<(), ErrorStackAdapter> {
        let client = Client::new();

        let requests = SatelliteDataSource::generate_requests(&client)?;

        let groups = SatelliteDataSource::execute_requests(requests, &client).await?;

        info!("Processing Elements");
        let mut elements: HashMap<ElementsAdapter, BTreeSet<&'static Group>> = HashMap::new();

        groups.into_iter().for_each(|(group, els)| {
            els.into_iter().for_each(|el| {
                let set = elements.entry(el).or_insert_with(BTreeSet::new);
                set.insert(group);
            });
        });

        let satellites = elements
            .into_iter()
            .map(|(el, groups)| Satellite::new(el.into(), groups))
            .collect::<error_stack::Result<BTreeSet<_>, SatelliteError>>();

        match satellites {
            Ok(sats) => {
                info!("Satellite collection has {} members", sats.len());
                self.add_sats_to_entity_collection(sats);
            }
            Err(e) => error!("{e}"),
        }

        Ok(())
    }
    fn generate_requests(
        client: &Client,
    ) -> error_stack::Result<Vec<(Request, &'static Group)>, Error> {
        let url = Url::from_str(BASE_URL)
            .into_report()
            .change_context(Error::GetSats)
            .attach_printable(format!("Couldn't parse {BASE_URL} as a URL"))?;

        let window = window()
            .ok_or(Error::GetSats)
            .into_report()
            .attach_printable("Couldn't get window reference")?;

        let origin = window.origin();

        let builder = client
            .get(url)
            .query(&[("FORMAT", "TLE")])
            .header("origin", &origin);

        trace!("load_data: Generating requests");
        let requests: Vec<_> = CATEGORIES
            .iter()
            .flat_map(|category| category.groups())
            .filter_map(|group: &Group| {
                let request = builder
                    .try_clone()
                    .ok_or(
                        Report::new(Error::GetSats)
                            .attach_printable("Was attempting to clone request Builder"),
                    )
                    .and_then(|builder| {
                        builder
                            .query(&[("GROUP", group.id())])
                            .build()
                            .into_report()
                            .change_context(Error::GetSats)
                            .attach_printable(group.name())
                    });

                match request {
                    Ok(request) => Some((request, group)),
                    Err(e) => {
                        error!("Couldn't build request: {e}");
                        None
                    }
                }
            })
            .collect();
        //.map_ok(|(request, group)| (client.execute(request), group))

        Ok(requests)
    }

    async fn execute_requests(
        requests: Vec<(Request, &'static Group)>,
        client: &Client,
    ) -> error_stack::Result<HashMap<&'static Group, HashSet<ElementsAdapter>>, Error> {
        let mut groups: HashMap<&'static Group, HashSet<ElementsAdapter>> = HashMap::new();

        trace!("load_data: Executing requests");
        for (request, group) in requests {
            let name = group.name();
            trace!("load_data: Beginning request for group {}", group.name());
            let elements: HashSet<ElementsAdapter> = client
                .execute(request)
                .await
                .into_report()
                .change_context(Error::GetSats)
                .attach_printable(format!("Couldn't TLE Data for group {name}"))?
                .text()
                .await
                .inspect(|_| info!("Got TLE Data for group {name}"))
                .into_report()
                .change_context(Error::GetSats)
                .attach_printable(format!("Couldn't get body data for group {name}"))?
                .split("\r\n")
                .map(|line| line.trim())
                .chunks(LINES_PER_TLE)
                .into_iter()
                .filter_map(|chunk| {
                    let lines: Vec<_> = chunk.take(LINES_PER_TLE).collect();
                    match lines.len() {
                        3 => Some(lines),
                        _ => None,
                    }
                })
                .map(|tle_lines: Vec<&str>| {
                    trace!("Parsing TLE {tle_lines:#?}");

                    let elements = Elements::from_tle(
                        Some(tle_lines[0].to_string()),
                        tle_lines[1].as_bytes(),
                        tle_lines[2].as_bytes(),
                    )
                    .to_sgp4_report()
                    .change_context(Error::GetSats);
                    elements
                })
                .filter_map(|el| match el {
                    Ok(el) => Some(el),
                    Err(e) => {
                        error!("{e:#?}");
                        None
                    }
                })
                .map_into()
                .collect();

            groups.insert(group, elements);
        }
        info!("All groups fetched");
        Ok(groups)
    }

    fn add_sats_to_entity_collection(&mut self, sats: BTreeSet<Satellite>) {
        let sats: BTreeSet<Satellite> = sats
            .into_iter()
            .map(|sat| {
                EntityCollection::add(&self.entities, sat.entity());
                sat
            })
            .collect();
        self.satellites = Some(sats);
        self.is_loading = false;
    }
}
