#[derive(Debug, Clone)]
pub struct SclDocument {
    pub header: SclHeader,
    pub ieds: Vec<Ied>,
}

#[derive(Debug, Clone)]
pub struct SclHeader {
    pub id: String,
    pub version: String,
    pub revision: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ied {
    pub name: String,
    pub desc: Option<String>,
    pub manufacturer: Option<String>,
    pub access_points: Vec<AccessPoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccessPoint {
    pub name: String,
    pub server: Option<Server>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Server {
    pub l_devices: Vec<LDevice>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LDevice {
    pub inst: String,
    pub ln0: LogicalNode,
    pub lns: Vec<LogicalNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalNode {
    pub ln_class: String,
    pub inst: String,
    pub prefix: Option<String>,
    pub ln_type: String,
    pub datasets: Vec<DataSet>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataSet {
    pub name: String,
    pub fcdas: Vec<Fcda>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fcda {
    pub ld_inst: String,
    pub prefix: Option<String>,
    pub ln_class: String,
    pub ln_inst: Option<String>,
    pub do_name: String,
    pub da_name: Option<String>,
    pub fc: String,
}
