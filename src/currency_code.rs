//! ISO 4217 alphabetic currency codes, from list one as published by SIX on
//! 2026-09-17: <https://www.six-group.com/en/products-services/financial-information/data-standards.html>

use crate::{Problem, text_value};

struct IsoCurrency {
    code: &'static str,
    numeric: &'static str,
    name: &'static str,
    minor_units: Option<u8>,
}

const CURRENCIES: &[IsoCurrency] = &[
    IsoCurrency {
        code: "AED",
        numeric: "784",
        name: "UAE Dirham",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "AFN",
        numeric: "971",
        name: "Afghani",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "ALL",
        numeric: "008",
        name: "Lek",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "AMD",
        numeric: "051",
        name: "Armenian Dram",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "AOA",
        numeric: "973",
        name: "Kwanza",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "ARS",
        numeric: "032",
        name: "Argentine Peso",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "AUD",
        numeric: "036",
        name: "Australian Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "AWG",
        numeric: "533",
        name: "Aruban Florin",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "AZN",
        numeric: "944",
        name: "Azerbaijan Manat",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BAM",
        numeric: "977",
        name: "Convertible Mark",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BBD",
        numeric: "052",
        name: "Barbados Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BDT",
        numeric: "050",
        name: "Taka",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BHD",
        numeric: "048",
        name: "Bahraini Dinar",
        minor_units: Some(3),
    },
    IsoCurrency {
        code: "BIF",
        numeric: "108",
        name: "Burundi Franc",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "BMD",
        numeric: "060",
        name: "Bermudian Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BND",
        numeric: "096",
        name: "Brunei Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BOB",
        numeric: "068",
        name: "Boliviano",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BOV",
        numeric: "984",
        name: "Mvdol",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BRL",
        numeric: "986",
        name: "Brazilian Real",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BSD",
        numeric: "044",
        name: "Bahamian Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BTN",
        numeric: "064",
        name: "Ngultrum",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BWP",
        numeric: "072",
        name: "Pula",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BYN",
        numeric: "933",
        name: "Belarusian Ruble",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "BZD",
        numeric: "084",
        name: "Belize Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "CAD",
        numeric: "124",
        name: "Canadian Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "CDF",
        numeric: "976",
        name: "Congolese Franc",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "CHE",
        numeric: "947",
        name: "WIR Euro",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "CHF",
        numeric: "756",
        name: "Swiss Franc",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "CHW",
        numeric: "948",
        name: "WIR Franc",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "CLF",
        numeric: "990",
        name: "Unidad de Fomento",
        minor_units: Some(4),
    },
    IsoCurrency {
        code: "CLP",
        numeric: "152",
        name: "Chilean Peso",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "CNY",
        numeric: "156",
        name: "Yuan Renminbi",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "COP",
        numeric: "170",
        name: "Colombian Peso",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "COU",
        numeric: "970",
        name: "Unidad de Valor Real",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "CRC",
        numeric: "188",
        name: "Costa Rican Colon",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "CUP",
        numeric: "192",
        name: "Cuban Peso",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "CVE",
        numeric: "132",
        name: "Cabo Verde Escudo",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "CZK",
        numeric: "203",
        name: "Czech Koruna",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "DJF",
        numeric: "262",
        name: "Djibouti Franc",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "DKK",
        numeric: "208",
        name: "Danish Krone",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "DOP",
        numeric: "214",
        name: "Dominican Peso",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "DZD",
        numeric: "012",
        name: "Algerian Dinar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "EGP",
        numeric: "818",
        name: "Egyptian Pound",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "ERN",
        numeric: "232",
        name: "Nakfa",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "ETB",
        numeric: "230",
        name: "Ethiopian Birr",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "EUR",
        numeric: "978",
        name: "Euro",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "FJD",
        numeric: "242",
        name: "Fiji Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "FKP",
        numeric: "238",
        name: "Falkland Islands Pound",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "GBP",
        numeric: "826",
        name: "Pound Sterling",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "GEL",
        numeric: "981",
        name: "Lari",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "GHS",
        numeric: "936",
        name: "Ghana Cedi",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "GIP",
        numeric: "292",
        name: "Gibraltar Pound",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "GMD",
        numeric: "270",
        name: "Dalasi",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "GNF",
        numeric: "324",
        name: "Guinean Franc",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "GTQ",
        numeric: "320",
        name: "Quetzal",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "GYD",
        numeric: "328",
        name: "Guyana Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "HKD",
        numeric: "344",
        name: "Hong Kong Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "HNL",
        numeric: "340",
        name: "Lempira",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "HTG",
        numeric: "332",
        name: "Gourde",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "HUF",
        numeric: "348",
        name: "Forint",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "IDR",
        numeric: "360",
        name: "Rupiah",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "ILS",
        numeric: "376",
        name: "New Israeli Sheqel",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "INR",
        numeric: "356",
        name: "Indian Rupee",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "IQD",
        numeric: "368",
        name: "Iraqi Dinar",
        minor_units: Some(3),
    },
    IsoCurrency {
        code: "IRR",
        numeric: "364",
        name: "Iranian Rial",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "ISK",
        numeric: "352",
        name: "Iceland Krona",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "JMD",
        numeric: "388",
        name: "Jamaican Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "JOD",
        numeric: "400",
        name: "Jordanian Dinar",
        minor_units: Some(3),
    },
    IsoCurrency {
        code: "JPY",
        numeric: "392",
        name: "Yen",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "KES",
        numeric: "404",
        name: "Kenyan Shilling",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "KGS",
        numeric: "417",
        name: "Som",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "KHR",
        numeric: "116",
        name: "Riel",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "KMF",
        numeric: "174",
        name: "Comorian Franc",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "KPW",
        numeric: "408",
        name: "North Korean Won",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "KRW",
        numeric: "410",
        name: "Won",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "KWD",
        numeric: "414",
        name: "Kuwaiti Dinar",
        minor_units: Some(3),
    },
    IsoCurrency {
        code: "KYD",
        numeric: "136",
        name: "Cayman Islands Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "KZT",
        numeric: "398",
        name: "Tenge",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "LAK",
        numeric: "418",
        name: "Lao Kip",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "LBP",
        numeric: "422",
        name: "Lebanese Pound",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "LKR",
        numeric: "144",
        name: "Sri Lanka Rupee",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "LRD",
        numeric: "430",
        name: "Liberian Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "LSL",
        numeric: "426",
        name: "Loti",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "LYD",
        numeric: "434",
        name: "Libyan Dinar",
        minor_units: Some(3),
    },
    IsoCurrency {
        code: "MAD",
        numeric: "504",
        name: "Moroccan Dirham",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MDL",
        numeric: "498",
        name: "Moldovan Leu",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MGA",
        numeric: "969",
        name: "Malagasy Ariary",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MKD",
        numeric: "807",
        name: "Denar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MMK",
        numeric: "104",
        name: "Kyat",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MNT",
        numeric: "496",
        name: "Tugrik",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MOP",
        numeric: "446",
        name: "Pataca",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MRU",
        numeric: "929",
        name: "Ouguiya",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MUR",
        numeric: "480",
        name: "Mauritius Rupee",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MVR",
        numeric: "462",
        name: "Rufiyaa",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MWK",
        numeric: "454",
        name: "Malawi Kwacha",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MXN",
        numeric: "484",
        name: "Mexican Peso",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MXV",
        numeric: "979",
        name: "Mexican Unidad de Inversion (UDI)",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MYR",
        numeric: "458",
        name: "Malaysian Ringgit",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "MZN",
        numeric: "943",
        name: "Mozambique Metical",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "NAD",
        numeric: "516",
        name: "Namibia Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "NGN",
        numeric: "566",
        name: "Naira",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "NIO",
        numeric: "558",
        name: "Cordoba Oro",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "NOK",
        numeric: "578",
        name: "Norwegian Krone",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "NPR",
        numeric: "524",
        name: "Nepalese Rupee",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "NZD",
        numeric: "554",
        name: "New Zealand Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "OMR",
        numeric: "512",
        name: "Rial Omani",
        minor_units: Some(3),
    },
    IsoCurrency {
        code: "PAB",
        numeric: "590",
        name: "Balboa",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "PEN",
        numeric: "604",
        name: "Sol",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "PGK",
        numeric: "598",
        name: "Kina",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "PHP",
        numeric: "608",
        name: "Philippine Peso",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "PKR",
        numeric: "586",
        name: "Pakistan Rupee",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "PLN",
        numeric: "985",
        name: "Zloty",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "PYG",
        numeric: "600",
        name: "Guarani",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "QAR",
        numeric: "634",
        name: "Qatari Rial",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "RON",
        numeric: "946",
        name: "Romanian Leu",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "RSD",
        numeric: "941",
        name: "Serbian Dinar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "RUB",
        numeric: "643",
        name: "Russian Ruble",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "RWF",
        numeric: "646",
        name: "Rwanda Franc",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "SAR",
        numeric: "682",
        name: "Saudi Riyal",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SBD",
        numeric: "090",
        name: "Solomon Islands Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SCR",
        numeric: "690",
        name: "Seychelles Rupee",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SDG",
        numeric: "938",
        name: "Sudanese Pound",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SEK",
        numeric: "752",
        name: "Swedish Krona",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SGD",
        numeric: "702",
        name: "Singapore Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SHP",
        numeric: "654",
        name: "Saint Helena Pound",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SLE",
        numeric: "925",
        name: "Leone",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SOS",
        numeric: "706",
        name: "Somali Shilling",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SRD",
        numeric: "968",
        name: "Surinam Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SSP",
        numeric: "728",
        name: "South Sudanese Pound",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "STN",
        numeric: "930",
        name: "Dobra",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SVC",
        numeric: "222",
        name: "El Salvador Colon",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SYP",
        numeric: "760",
        name: "Syrian Pound",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "SZL",
        numeric: "748",
        name: "Lilangeni",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "THB",
        numeric: "764",
        name: "Baht",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "TJS",
        numeric: "972",
        name: "Somoni",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "TMT",
        numeric: "934",
        name: "Turkmenistan New Manat",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "TND",
        numeric: "788",
        name: "Tunisian Dinar",
        minor_units: Some(3),
    },
    IsoCurrency {
        code: "TOP",
        numeric: "776",
        name: "Pa’anga",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "TRY",
        numeric: "949",
        name: "Turkish Lira",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "TTD",
        numeric: "780",
        name: "Trinidad and Tobago Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "TWD",
        numeric: "901",
        name: "New Taiwan Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "TZS",
        numeric: "834",
        name: "Tanzanian Shilling",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "UAH",
        numeric: "980",
        name: "Hryvnia",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "UGX",
        numeric: "800",
        name: "Uganda Shilling",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "USD",
        numeric: "840",
        name: "US Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "USN",
        numeric: "997",
        name: "US Dollar (Next day)",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "UYI",
        numeric: "940",
        name: "Uruguay Peso en Unidades Indexadas (UI)",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "UYU",
        numeric: "858",
        name: "Peso Uruguayo",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "UYW",
        numeric: "927",
        name: "Unidad Previsional",
        minor_units: Some(4),
    },
    IsoCurrency {
        code: "UZS",
        numeric: "860",
        name: "Uzbekistan Sum",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "VED",
        numeric: "926",
        name: "Bolívar Soberano",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "VES",
        numeric: "928",
        name: "Bolívar Soberano",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "VND",
        numeric: "704",
        name: "Dong",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "VUV",
        numeric: "548",
        name: "Vatu",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "WST",
        numeric: "882",
        name: "Tala",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "XAD",
        numeric: "396",
        name: "Arab Accounting Dinar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "XAF",
        numeric: "950",
        name: "CFA Franc BEAC",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "XAG",
        numeric: "961",
        name: "Silver",
        minor_units: None,
    },
    IsoCurrency {
        code: "XAU",
        numeric: "959",
        name: "Gold",
        minor_units: None,
    },
    IsoCurrency {
        code: "XBA",
        numeric: "955",
        name: "Bond Markets Unit European Composite Unit (EURCO)",
        minor_units: None,
    },
    IsoCurrency {
        code: "XBB",
        numeric: "956",
        name: "Bond Markets Unit European Monetary Unit (E.M.U.-6)",
        minor_units: None,
    },
    IsoCurrency {
        code: "XBC",
        numeric: "957",
        name: "Bond Markets Unit European Unit of Account 9 (E.U.A.-9)",
        minor_units: None,
    },
    IsoCurrency {
        code: "XBD",
        numeric: "958",
        name: "Bond Markets Unit European Unit of Account 17 (E.U.A.-17)",
        minor_units: None,
    },
    IsoCurrency {
        code: "XCD",
        numeric: "951",
        name: "East Caribbean Dollar",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "XCG",
        numeric: "532",
        name: "Caribbean Guilder",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "XDR",
        numeric: "960",
        name: "SDR (Special Drawing Right)",
        minor_units: None,
    },
    IsoCurrency {
        code: "XOF",
        numeric: "952",
        name: "CFA Franc BCEAO",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "XPD",
        numeric: "964",
        name: "Palladium",
        minor_units: None,
    },
    IsoCurrency {
        code: "XPF",
        numeric: "953",
        name: "CFP Franc",
        minor_units: Some(0),
    },
    IsoCurrency {
        code: "XPT",
        numeric: "962",
        name: "Platinum",
        minor_units: None,
    },
    IsoCurrency {
        code: "XSU",
        numeric: "994",
        name: "Sucre",
        minor_units: None,
    },
    IsoCurrency {
        code: "XTS",
        numeric: "963",
        name: "Codes specifically reserved for testing purposes",
        minor_units: None,
    },
    IsoCurrency {
        code: "XUA",
        numeric: "965",
        name: "ADB Unit of Account",
        minor_units: None,
    },
    IsoCurrency {
        code: "XXX",
        numeric: "999",
        name: "The codes assigned for transactions where no currency is involved",
        minor_units: None,
    },
    IsoCurrency {
        code: "YER",
        numeric: "886",
        name: "Yemeni Rial",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "ZAR",
        numeric: "710",
        name: "Rand",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "ZMW",
        numeric: "967",
        name: "Zambian Kwacha",
        minor_units: Some(2),
    },
    IsoCurrency {
        code: "ZWG",
        numeric: "924",
        name: "Zimbabwe Gold",
        minor_units: Some(2),
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CurrencyCode(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrencyCodeProblem {
    Empty,
    Unknown,
}

impl Problem for CurrencyCodeProblem {
    fn message(&self) -> String {
        match self {
            Self::Empty => "Choose a currency.".to_owned(),
            Self::Unknown => "Choose a currency from the list.".to_owned(),
        }
    }
}

fn currency(code: &str) -> Option<&'static IsoCurrency> {
    CURRENCIES.iter().find(|candidate| candidate.code == code)
}

impl CurrencyCode {
    pub fn parse(typed: &str) -> Result<Self, CurrencyCodeProblem> {
        let code = typed.trim().to_uppercase();
        if code.is_empty() {
            return Err(CurrencyCodeProblem::Empty);
        }
        currency(&code)
            .map(|_| Self(code))
            .ok_or(CurrencyCodeProblem::Unknown)
    }

    pub fn parse_optional(typed: &str) -> Result<Option<Self>, CurrencyCodeProblem> {
        if typed.trim().is_empty() {
            return Ok(None);
        }
        Self::parse(typed).map(Some)
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        currency(&self.0).map_or("", |currency| currency.name)
    }

    #[must_use]
    pub fn numeric_code(&self) -> &'static str {
        currency(&self.0).map_or("", |currency| currency.numeric)
    }

    #[must_use]
    pub fn minor_units(&self) -> Option<u8> {
        currency(&self.0).and_then(|currency| currency.minor_units)
    }

    pub fn all() -> impl Iterator<Item = Self> {
        CURRENCIES
            .iter()
            .map(|currency| Self(currency.code.to_owned()))
    }
}

text_value!(CurrencyCode, CurrencyCodeProblem);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holds_every_iso_code_once() {
        let mut codes: Vec<&str> = CURRENCIES.iter().map(|currency| currency.code).collect();
        assert!(
            codes.iter().all(|code| code.len() == 3
                && code.chars().all(|character| character.is_ascii_uppercase()))
        );
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), CURRENCIES.len());
        assert_eq!(CurrencyCode::all().count(), 178);
    }

    #[test]
    fn accepts_iso_codes_in_any_case() {
        let dollar = CurrencyCode::parse(" usd ").unwrap();
        assert_eq!(dollar.as_str(), "USD");
        assert_eq!(dollar.name(), "US Dollar");
        assert_eq!(dollar.numeric_code(), "840");
    }

    #[test]
    fn knows_how_many_decimal_places_each_currency_uses() {
        assert_eq!(CurrencyCode::parse("GBP").unwrap().minor_units(), Some(2));
        assert_eq!(CurrencyCode::parse("JPY").unwrap().minor_units(), Some(0));
        assert_eq!(CurrencyCode::parse("BHD").unwrap().minor_units(), Some(3));
        assert_eq!(CurrencyCode::parse("XAU").unwrap().minor_units(), None);
    }

    #[test]
    fn rejects_withdrawn_and_unassigned_codes() {
        for rejected in ["BGN", "DEM", "ABC", "US", "USDT"] {
            assert_eq!(
                CurrencyCode::parse(rejected),
                Err(CurrencyCodeProblem::Unknown),
                "{rejected}"
            );
        }
    }

    #[test]
    fn blank_is_empty_or_absent() {
        assert_eq!(CurrencyCode::parse("  "), Err(CurrencyCodeProblem::Empty));
        assert_eq!(CurrencyCode::parse_optional("  "), Ok(None));
    }
}
