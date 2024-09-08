use crate::currency::Currency;
use crate::definitions::Real;
use crate::Tenor;
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use static_id::static_id::StaticId;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SurfaceData {
    pub spot: Option<Real>,
    pub value: Array2<Real>,
    pub dates: Vec<OffsetDateTime>,
    pub strikes: Array1<Real>,
    pub currency: Currency,
    pub market_datetime: Option<OffsetDateTime>,
    pub name: String,
    pub id: StaticId,
}

impl SurfaceData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        spot: Option<Real>,
        value: Array2<Real>,
        dates: Vec<OffsetDateTime>,
        strikes: Array1<Real>,
        market_datetime: Option<OffsetDateTime>,
        currency: Currency,
        name: String,
        id: StaticId,
    ) -> SurfaceData {
        SurfaceData {
            spot,
            value,
            dates,
            strikes,
            market_datetime,
            currency,
            name,
            id, 
        }
    }

    pub fn set_spot(&mut self, spot: Option<Real>) {
        self.spot = spot;
    }

    pub fn get_spot(&self) -> Option<Real> {
        self.spot
    }

    pub fn get_value(&self) -> &Array2<Real> {
        &self.value
    }

    pub fn get_dates(&self) -> &Vec<OffsetDateTime> {
        &self.dates
    }

    pub fn get_market_datetime(&self) -> Option<OffsetDateTime> {
        self.market_datetime
    }

    pub fn get_strike(&self) -> &Array1<Real> {
        &self.strikes
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_id(&self) -> StaticId {
        self.id
    }

    pub fn test_data(spot: Real, datetime: Option<OffsetDateTime>) -> Result<SurfaceData> {
        let datetime = datetime.unwrap_or_else(OffsetDateTime::now_utc);
        let dates = vec![
            Tenor::new_from_string("1M")?.apply(&datetime),
            Tenor::new_from_string("2M")?.apply(&datetime),
            Tenor::new_from_string("3M")?.apply(&datetime),
            Tenor::new_from_string("6M")?.apply(&datetime),
            Tenor::new_from_string("9M")?.apply(&datetime),
            Tenor::new_from_string("1Y")?.apply(&datetime),
            Tenor::new_from_string("1Y6M")?.apply(&datetime),
            Tenor::new_from_string("2Y")?.apply(&datetime),
            Tenor::new_from_string("3Y")?.apply(&datetime),
        ];

        let currency = Currency::KRW;
        let id = StaticId::from_str("KOSPI2 20220414 Data", "KAP");
        let value = Array2::from_shape_vec(
            (9, 25),
            #[allow(clippy::unreadable_literal)]
            vec![
                [
                    0.904_032, 0.821_533, 0.748_239, 0.681_917, 0.621_013, 0.564_395,
                    0.511_195, 0.460_727, 0.412_415, 0.365_763, 0.320_319, 0.275_673,
                    0.231_523, 0.188_142, 0.149_353, 0.132_981, 0.145_66, 0.167_21,
                    0.189_804, 0.211_774, 0.232_762, 0.252_733, 0.271_736, 0.289_845,
                    0.307_132,
                ],
                [
                    0.757_368, 0.691_392, 0.632_752, 0.579_675, 0.530_931, 0.485_626,
                    0.443_082, 0.402_768, 0.364_255, 0.327_19, 0.291_288, 0.256_356,
                    0.222_404, 0.190_057, 0.162_026, 0.145_896, 0.147_238, 0.158_936,
                    0.174_041, 0.189_881, 0.205_56, 0.220_769, 0.235_411, 0.249_468,
                    0.262_957,
                ],
                [
                    0.682_848, 0.625_079, 0.573_729, 0.527_255, 0.484_585, 0.444_944,
                    0.407_75, 0.372_549, 0.338_987, 0.306_783, 0.275_735, 0.245_753,
                    0.216_968, 0.190_073, 0.167_197, 0.152_949, 0.150_916, 0.157_908,
                    0.168_979, 0.181_475, 0.194_288, 0.206_962, 0.219_311, 0.231_261,
                    0.242_79,
                ],
                [
                    0.571_081, 0.525_578, 0.485_156, 0.448_607, 0.415_096, 0.384_026,
                    0.354_951, 0.327_538, 0.301_536, 0.276_768, 0.253_133, 0.230_639,
                    0.209_472, 0.190_148, 0.173_76, 0.162_083, 0.156_705, 0.157_371,
                    0.162_186, 0.169_278, 0.177_461, 0.186_092, 0.194_832, 0.203_504,
                    0.212_017,
                ],
                [
                    0.512_868, 0.473_601, 0.438_747, 0.407_264, 0.378_439, 0.351_762,
                    0.326_858, 0.303_451, 0.281_341, 0.260_397, 0.240_557, 0.221_858,
                    0.204_473, 0.188_793, 0.175_52, 0.165_639, 0.160_015, 0.158_71,
                    0.160_85, 0.165_264, 0.171_004, 0.177_453, 0.184_237, 0.191_139,
                    0.198_031,
                ],
                [
                    0.474_875, 0.439_666, 0.408_439, 0.380_263, 0.354_501, 0.330_7,
                    0.308_531, 0.287_755, 0.268_203, 0.249_769, 0.232_416, 0.216_186,
                    0.201_233, 0.187_864, 0.176_57, 0.167_976, 0.162_608, 0.160_544,
                    0.161_31, 0.164_144, 0.168_331, 0.173_336, 0.178_802, 0.184_501,
                    0.190_291,
                ],
                [
                    0.426_607, 0.396_703, 0.370_229, 0.346_395, 0.324_664, 0.304_655,
                    0.286_097, 0.268_796, 0.252_619, 0.237_489, 0.223_383, 0.210_337,
                    0.198_457, 0.187_928, 0.179_012, 0.172_002, 0.167_13, 0.164_449,
                    0.163_775, 0.164_752, 0.166_969, 0.170_056, 0.173_723, 0.177_759,
                    0.182_015,
                ],
                [
                    0.393_157, 0.366_712, 0.343_341, 0.322_347, 0.303_257, 0.285_74,
                    0.269_562, 0.254_558, 0.240_622, 0.227_693, 0.215_761, 0.204_859,
                    0.195_071, 0.186_525, 0.179_379, 0.173_785, 0.169_836, 0.167_516,
                    0.166_687, 0.167_119, 0.168_548, 0.170_726, 0.173_44, 0.176_527,
                    0.179_861,
                ],
                [
                    0.348_153, 0.326_246, 0.306_965, 0.289_731, 0.274_156, 0.259_973,
                    0.246_998, 0.235_106, 0.224_22, 0.214_3, 0.205_343, 0.197_37,
                    0.190_426, 0.184_562, 0.179_822, 0.176_225, 0.173_743, 0.172_305,
                    0.171_795, 0.172_074, 0.172_998, 0.174_431, 0.176_255, 0.178_371,
                    0.180_702,
                ],
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        ).expect("Failed to create Array2 from Vec");

        let strikes = Array1::linspace(
            0.3 * spot, 1.5 * spot, 25
        );
        Ok(SurfaceData {
            spot: Some(spot), 
            value,
            dates,
            strikes,
            market_datetime: Some(datetime),
            currency,
            name: id.code_str().to_string(),
            id,
        })
    }
}

impl Default for SurfaceData {
    fn default() -> Self {
        SurfaceData {
            spot: None,
            value: Array2::zeros((0, 0)),
            dates: Vec::new(),
            strikes: Array1::zeros(0),
            market_datetime: None,
            currency: Currency::default(),
            name: String::new(),
            id: StaticId::default(),
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;
    #[test]
    fn test_surface_data() {
        let surface_data = SurfaceData::test_data(
            100.0, Some(datetime!(2022-04-14 15:40:00 +09:00))).unwrap();
        println!("{:?}", surface_data);
    }
}