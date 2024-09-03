use crate::currency::Currency;
use crate::definitions::Real;
use crate::Tenor;
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use static_id::StaticId;
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
        let datetime = datetime.unwrap_or_else(|| OffsetDateTime::now_utc());
        let tenor_vec = vec![
            Tenor::new_from_string("1M")?,
            Tenor::new_from_string("2M")?,
            Tenor::new_from_string("3M")?,
            Tenor::new_from_string("6M")?,
            Tenor::new_from_string("9M")?,
            Tenor::new_from_string("1Y")?,
            Tenor::new_from_string("1Y6M")?,
            Tenor::new_from_string("2Y")?,
            Tenor::new_from_string("3Y")?,
        ];

        let dates = tenor_vec
            .iter()
            .map(|tenor| tenor.apply(&datetime))
            .collect::<Vec<_>>();

        let currency = Currency::KRW;
        let id = StaticId::from_str("KOSPI2 20220414 Data", "KAP");
        let value = Array2::from_shape_vec(
            (9, 25),
            vec![
                [
                    0.9040320, 0.8215330, 0.7482390, 0.6819170, 0.6210130, 0.5643950,
                    0.5111950, 0.4607270, 0.4124150, 0.3657630, 0.3203190, 0.2756730,
                    0.2315230, 0.1881420, 0.1493530, 0.1329810, 0.1456600, 0.1672100,
                    0.1898040, 0.2117740, 0.2327620, 0.2527330, 0.2717360, 0.2898450,
                    0.3071320,
                ],
                [
                    0.7573680, 0.6913920, 0.6327520, 0.5796750, 0.5309310, 0.4856260,
                    0.4430820, 0.4027680, 0.3642550, 0.3271900, 0.2912880, 0.2563560,
                    0.2224040, 0.1900570, 0.1620260, 0.1458960, 0.1472380, 0.1589360,
                    0.1740410, 0.1898810, 0.2055600, 0.2207690, 0.2354110, 0.2494680,
                    0.2629570,
                ],
                [
                    0.6828480, 0.6250790, 0.5737290, 0.5272550, 0.4845850, 0.4449440,
                    0.4077500, 0.3725490, 0.3389870, 0.3067830, 0.2757350, 0.2457530,
                    0.2169680, 0.1900730, 0.1671970, 0.1529490, 0.1509160, 0.1579080,
                    0.1689790, 0.1814750, 0.1942880, 0.2069620, 0.2193110, 0.2312610,
                    0.2427900,
                ],
                [
                    0.5710810, 0.5255780, 0.4851560, 0.4486070, 0.4150960, 0.3840260,
                    0.3549510, 0.3275380, 0.3015360, 0.2767680, 0.2531330, 0.2306390,
                    0.2094720, 0.1901480, 0.1737600, 0.1620830, 0.1567050, 0.1573710,
                    0.1621860, 0.1692780, 0.1774610, 0.1860920, 0.1948320, 0.2035040,
                    0.2120170,
                ],
                [
                    0.5128680, 0.4736010, 0.4387470, 0.4072640, 0.3784390, 0.3517620,
                    0.3268580, 0.3034510, 0.2813410, 0.2603970, 0.2405570, 0.2218580,
                    0.2044730, 0.1887930, 0.1755200, 0.1656390, 0.1600150, 0.1587100,
                    0.1608500, 0.1652640, 0.1710040, 0.1774530, 0.1842370, 0.1911390,
                    0.1980310,
                ],
                [
                    0.4748750, 0.4396660, 0.4084390, 0.3802630, 0.3545010, 0.3307000,
                    0.3085310, 0.2877550, 0.2682030, 0.2497690, 0.2324160, 0.2161860,
                    0.2012330, 0.1878640, 0.1765700, 0.1679760, 0.1626080, 0.1605440,
                    0.1613100, 0.1641440, 0.1683310, 0.1733360, 0.1788020, 0.1845010,
                    0.1902910,
                ],
                [
                    0.4266070, 0.3967030, 0.3702290, 0.3463950, 0.3246640, 0.3046550,
                    0.2860970, 0.2687960, 0.2526190, 0.2374890, 0.2233830, 0.2103370,
                    0.1984570, 0.1879280, 0.1790120, 0.1720020, 0.1671300, 0.1644490,
                    0.1637750, 0.1647520, 0.1669690, 0.1700560, 0.1737230, 0.1777590,
                    0.1820150,
                ],
                [
                    0.3931570, 0.3667120, 0.3433410, 0.3223470, 0.3032570, 0.2857400,
                    0.2695620, 0.2545580, 0.2406220, 0.2276930, 0.2157610, 0.2048590,
                    0.1950710, 0.1865250, 0.1793790, 0.1737850, 0.1698360, 0.1675160,
                    0.1666870, 0.1671190, 0.1685480, 0.1707260, 0.1734400, 0.1765270,
                    0.1798610,
                ],
                [
                    0.3481530, 0.3262460, 0.3069650, 0.2897310, 0.2741560, 0.2599730,
                    0.2469980, 0.2351060, 0.2242200, 0.2143000, 0.2053430, 0.1973700,
                    0.1904260, 0.1845620, 0.1798220, 0.1762250, 0.1737430, 0.1723050,
                    0.1717950, 0.1720740, 0.1729980, 0.1744310, 0.1762550, 0.1783710,
                    0.1807020,
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