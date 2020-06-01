use super::schema::FmcApi;
use uuid::Uuid;

impl FmcApi {
    pub(super) async fn path_vec(self, address: &str, dom_uuid: Uuid) -> Vec<String> {
        let mut path: Vec<String> = Vec::new();
        path.push("https://".to_string());
        path.push(address.to_string());
        path.push(":443".to_string());
        
        match self {
            Self::Devices => { path.push("/api/fmc_config/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/devices".to_string());
                                path },
            Self::PolicyAssignments => { path.push("/api/fmc_config/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/assignments/policyassignments".to_string());
                                path }, 
            Self::DeviceHAPairs => { path.push("/api/fmc_config/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/devicehapairs/ftddevicehapairs".to_string());
                                path },
            Self::Integration => { path.push("/api/fmc_config/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/integration".to_string());
                                path }, 
            Self::DeviceGroups => { path.push("/api/fmc_config/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/devicegroups/devicegrouprecords".to_string());
                                path },
            Self::TaskStatuses => { path.push("/api/fmc_config/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/taskstatuses".to_string());
                                path },
            Self::DeviceClusters => { path.push("/api/fmc_config/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/devices".to_string());
                                path },
            Self::Object => { path.push("/api/fmc_config/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/object".to_string());
                                path }, 
            Self::Policy => { path.push("/api/fmc_config/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/policy".to_string());
                                path },
            Self::Deployment => { path.push("/api/fmc_config/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/deployment".to_string());
                                path },
            Self::Updates => { path.push("/api/fmc_config/v1/domain/".to_string());
                                path },
            Self::Audit => { path.push("/api/fmc_platform/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/audit/auditrecords".to_string());
                                path }, 
            Self::Info => { path.push("/api/fmc_platform/v1/info".to_string());
                                path}, 
            Self::TaxiiConfig => { path.push("/api/fmc_tid/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/taxiiconfig".to_string());
                                path }, 
            Self::Tid => { path.push("/api/fmc_tid/v1/domain/".to_string()); 
                                path.push(dom_uuid.to_string());
                                path.push("/tid".to_string());
                                path },  
        }
    }

    pub(super) async fn path_string (self, address: &str, dom_uuid: Uuid) -> String {
        self.path_vec(address, dom_uuid).await.join("")
    }
}