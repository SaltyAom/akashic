use async_graphql::{ SimpleObject, ComplexObject };

use serde::Serialize;
use super::service::{ get_nhql_comment, get_nhql_related };

#[derive(Serialize, Clone, SimpleObject)]
pub struct NHResponse {
    pub success: bool,
    pub info: Option<&'static str>,
    pub data: Option<Nhql>
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct NHSearchResponse {
    pub success: bool,
    pub info: Option<&'static str>,
    pub data: NhqlSearch
}

#[derive(Serialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Nhql {
    pub id: u32,
    pub title: NhqlTitle,
    pub images: NhqlImages,
    pub info: NhqlInfo,
    pub metadata: NhqlMetadata
}

#[ComplexObject]
impl Nhql {
    pub async fn comments(
        &self, 
        from: Option<u32>,     
        to: Option<u32>,
        batch: Option<u32>,
        batch_by: Option<u32>
    ) -> Vec<NhqlComment> {
        get_nhql_comment(self.id, from, to, batch, batch_by).await
    }

    pub async fn related(&self) -> Vec<Nhql> {
        get_nhql_related(self.id).await
    }
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct NhqlTitle {
    pub display: Option<String>,
    pub english: Option<String>,
    pub japanese: Option<String>
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct NhqlImages {
    pub pages: NhqlPages,
    pub cover: NhqlPage
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct NhqlPage {
    pub link: String,
    pub info: NhqlPageInfo
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct NhqlPageInfo {
    pub r#type: &'static str,
    pub width: u16,
    pub height: u16
}

pub type NhqlPages = Vec<NhqlPage>;

#[derive(Serialize, Clone, SimpleObject)]
pub struct NhqlInfo {
    pub amount: u32,
    pub favorite: u32,
    pub upload: u32
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct NhqlMetadata {
    pub artist: NhqlArtist,
    pub tags: NhqlTags,
    pub language: String
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct NhqlArtist {
    pub name: String,
    pub count: u32,
    pub url: String
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct NhqlTag {
    pub name: String,
    pub count: u32,
    pub url: String
}

pub type NhqlTags = Vec<NhqlTag>;
pub type NhqlSearch = Vec<Nhql>;

#[derive(Serialize, Clone, SimpleObject)]
pub struct NhqlComment {
    pub id: u32,
    pub user: NhqlUser,
    pub created: u32,
    pub body: String
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct NhqlUser {
    pub id: u32,
    pub username: String,
    pub slug: String,
    pub avatar: String
}