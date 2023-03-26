use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    str::FromStr,
};

use itertools::Itertools;
use log::{debug, error, info, trace};
use reqwest_wasm::{Client, Url};
use result_inspect::{ResultInspect, ResultInspectErr};
use sgp4::Elements;
use wasm_bindgen::prelude::*;

use error_stack::{IntoReport, Report, Result, ResultExt};
use web_sys::window;

use crate::{
    data::{group::Group, static_data::CATEGORIES},
    data_source::{data_fetching::adapter::ElementsAdapter, SatelliteDataSource},
    error::{self, Error, WrapSgp4Error},
};

mod adapter;

static BASE_URL: &str = "https://celestrak.org/NORAD/elements/gp.php";

#[wasm_bindgen]
impl SatelliteDataSource {
    #[wasm_bindgen]
    pub async fn load_data(&mut self) {
        debug!("Loading Satellite Data");
        let client: Client = reqwest_wasm::Client::builder()
            .build()
            .into_report()
            .change_context(Error::GetSats)
            .unwrap_throw();

        let url = Url::from_str(BASE_URL)
            .into_report()
            .change_context(Error::GetSats)
            .expect_throw(&format!("Couldn't parse {BASE_URL} as a URL"));

        let window = window().expect_throw("Couldn't get window reference");

        let origin = window.origin();

        let builder = client
            .get(url)
            .query(&[("FORMAT", "TLE")])
            .header("origin", &origin);

        trace!("load_data: Generating requests");
        let requests: Vec<_> = CATEGORIES
            .iter()
            .flat_map(|category| category.groups())
            .map(|group| {
                let request = builder
                    .try_clone()
                    .ok_or(Report::new(Error::GetSats))
                    .expect_throw("Couldn't clone request builder")
                    .query(&[("GROUP", group.id())])
                    .build()
                    .into_report()
                    .change_context(Error::GetSats)
                    .expect_throw(&format!("Couldn't build request for {}", group.name()));
                (request, group)
            })
            .map(|(request, group)| (client.execute(request), group))
            .collect();

        let mut groups: HashMap<&Group, HashSet<ElementsAdapter>> = HashMap::new();

        trace!("load_data: Executing requests");
        for (request, group) in requests {
            let name = group.name();
            trace!("load_data: Beginning request for group {}", group.name());
            let elements: HashSet<ElementsAdapter> = request
                .await
                .into_report()
                .change_context(Error::GetSats)
                .expect_throw(&format!("Couldn't TLE Data for group {name}"))
                .text()
                .await
                .into_report()
                .change_context(Error::GetSats)
                .inspect(|_| info!("Loaded TLE data for group {name}"))
                .expect_throw(&format!("Couldn't get body data for group {name}"))
                .split("\r\n")
                .map(|line| line.trim())
                .chunks(3)
                .into_iter()
                .map(|chunk| chunk.take(3).collect())
                .filter(|lines: &Vec<&str>| lines.len() == 3)
                .map(|tle_lines: Vec<&str>| {
                    trace!("Parsing TLE {tle_lines:#?}");

                    let elements = Elements::from_tle(
                        Some(tle_lines[0].to_string()),
                        tle_lines[1].as_bytes(),
                        tle_lines[2].as_bytes(),
                    )
                    .to_sgp4_report()
                    .change_context(Error::GetSats);
                    elements.unwrap()
                })
                .map_into()
                .collect();

            groups.insert(group, elements);
        }
        info!("All groups fetched");
        todo!()
    }
}
