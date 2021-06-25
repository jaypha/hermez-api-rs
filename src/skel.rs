use serde::Deserialize;
use ureq::Agent;
use url::Url;

use super::PaginationOrder;

pub fn get_xxx(
    agent: &Agent,
    url: &Url,
) -> Result<Xxx, super::ErrorKind> {
    let xxx: Xxx = agent
        .request_url(
            "GET",
            &url.join(&format!("xxxs/{}/{}", batch_num, account_index))
                .unwrap(),
        )
        .call()?
        .into_json()?;
    Ok(xxx)
}

pub struct XxxsGetOptions<'a> {
    agent: &'a Agent,
    url: &'a Url,

    from_item: Option<u64>,
    order: Option<PaginationOrder>,
    limit: Option<u64>,
}

impl<'a> XxxsGetOptions<'a> {
    pub fn new(agent: &'a Agent, url: &'a Url) -> Self {
        Self {
            agent,
            url,

            from_item: None,
            order: None
            limit: None,
        }
    }

    pagination_setters!();

    pub fn fetch(&self) -> Result<(Vec<Xxx>, u64), super::ErrorKind> {
        let mut url = self.url.join("xxxs").unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();

            pagination_fetch_stmts!(self, query_pairs);
        }
        let xxxs: Xxxs = self.agent.request_url("GET", &url).call()?.into_json()?;
        Ok((xxxs.xxxs, xxxs.pending_items))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Xxxs {
    pub pending_items: u64,
    pub xxxs: Vec<Xxx>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Xxx {
}
