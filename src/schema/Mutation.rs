use super::Requests::ReqInput;
use async_graphql::*;
use sqlx::{Pool, Postgres};
pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn initRequests<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        requests: Option<Vec<ReqInput>>,
    ) -> Result<bool> {
        let pool = ctx.data::<Pool<Postgres>>().unwrap();
        if let Some(requests) = &requests {
            sqlx::query("INSERT INTO requests(requestnumber,id,crewtransfercost,unittype,requeststatus,statusdescription,deliverydate,createdby,userid,addeddate,modifieddate,subunittype,subunittypearea,area,areatype,price,buildingarea,landarea) SELECT * FROM UNNEST ($1::text[],$2::int8[],$3::int8[],$4::int8[],$5::int8[],$6::text[],$7::date[],$8::text[],$9::text[],$10::timestamp[],$11::timestamp[],$12::int8[],$13::numeric[],$14::numeric[],$15::int8[],$16::numeric[],$17::text[],$18::text[])")
                .bind(&requests
                    .into_iter()
                    .map(|r| r.requestnumber.clone().unwrap())
                    .collect::<Vec<String>>()[..])
                .bind(&requests.into_iter().map(|r| r.id).collect::<Vec<i32>>()[..])
                .bind(&requests.into_iter().map(|r| r.crewtransfercost).collect::<Vec<Option<i32>>>()[..])
                .bind(&requests.into_iter().map(|r| r.unittype).collect::<Vec<Option<i32>>>()[..])
                .bind(&requests.into_iter().map(|r| r.requeststatus).collect::<Vec<Option<i32>>>()[..])
                .bind(&requests.into_iter().map(|r| r.statusdescription.clone()).collect::<Vec<Option<String>>>()[..])
                .bind(&requests.into_iter().map(|r| r.deliverydate.clone()).collect::<Vec<Option<String>>>()[..])
                .bind(&requests.into_iter().map(|r| r.createdby.clone()).collect::<Vec<Option<String>>>()[..])
                .bind(&requests.into_iter().map(|r| r.userid.clone()).collect::<Vec<Option<String>>>()[..])
                .bind(&requests.into_iter().map(|r| r.addeddate.clone()).collect::<Vec<Option<String>>>()[..])
                .bind(&requests.into_iter().map(|r| r.modifieddate.clone()).collect::<Vec<Option<String>>>()[..])
                .bind(&requests.into_iter().map(|r| r.subunittype).collect::<Vec<Option<i32>>>()[..])
                .bind(&requests.into_iter().map(|r| r.subunittypearea).collect::<Vec<Option<f64>>>()[..])
                .bind(&requests.into_iter().map(|r| r.area).collect::<Vec<Option<f64>>>()[..])
                .bind(&requests.into_iter().map(|r| r.areatype).collect::<Vec<Option<i32>>>()[..])
                .bind(&requests.into_iter().map(|r| r.price).collect::<Vec<Option<f64>>>()[..])
                .bind(&requests.into_iter().map(|r| r.buildingarea.clone()).collect::<Vec<Option<String>>>()[..])
                .bind(&requests.into_iter().map(|r| r.landarea.clone()).collect::<Vec<Option<String>>>()[..])





                .execute(pool)
                .await?;
        }

        // User signup
        Ok(true)
    }

    pub async fn login(&self, username: String, password: String) -> Result<String> {
        // User login (generate token)
        Ok("hi".to_string())
    }
}
