pub(crate) enum FmcApi {
    //Config: /api​/fmc_config​/v1​/domain​/f3b4958c-52a1-11e7-802a-010203040506​/{type}
    Devices,
    PolicyAssignments,
    DeviceHAPairs,
    Integration,
    DeviceGroups,
    TaskStatuses,
    DeviceClusters,
    Object,
    Policy,
    Deployment,

    //Platform: /api/fmc_platform/v1/{type}
    Updates,
    Audit,
    Info,

    //Threat intelligence: /api/fmc_tid/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/{type}
    TaxiiConfig,
    Tid,

    /// api/fmc_platform/v1/auth/generatetoken.
    HttpBasicAuth,
}

#[cfg(test)]
mod tests {
    use super::super::Uuid;
    use super::*;

    #[tokio::test]
    async fn fn_path_string() {
        //Test each enum variant
        let dom_uuid = Uuid::parse_str("f3b4958c-52a1-11e7-802a-010203040506").unwrap();

        assert_eq!(FmcApi::Devices.path_string("ciscofmc.local", Some(dom_uuid)).await, 
            "https://ciscofmc.local:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/devices");
        assert_eq!(FmcApi::Object.path_string("cisco_fmc.xyz", Some(dom_uuid)).await, 
            "https://cisco_fmc.xyz:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/object");
        assert_eq!(FmcApi::PolicyAssignments.path_string("fmc.abc", Some(dom_uuid)).await,
            "https://fmc.abc:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/assignments/policyassignments");
        assert_eq!(FmcApi::DeviceHAPairs.path_string("1.1.1.1", Some(dom_uuid)).await, 
            "https://1.1.1.1:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/devicehapairs/ftddevicehapairs");
        assert_eq!(FmcApi::Integration.path_string("10.0.0.5", Some(dom_uuid)).await,
            "https://10.0.0.5:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/integration");
        assert_eq!(FmcApi::DeviceGroups.path_string("fmc.test.domain", Some(dom_uuid)).await,
            "https://fmc.test.domain:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/devicegroups/devicegrouprecords");
        assert_eq!(FmcApi::TaskStatuses.path_string("lily123.cx", Some(dom_uuid)).await,
            "https://lily123.cx:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/taskstatuses");
        assert_eq!(FmcApi::DeviceClusters.path_string("AaBbCc.zzz", Some(dom_uuid)).await,    
            "https://AaBbCc.zzz:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/devices");
        assert_eq!(FmcApi::Object.path_string("fmc-01.sw.local", Some(dom_uuid)).await,
            "https://fmc-01.sw.local:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/object");
        assert_eq!(FmcApi::Policy.path_string("blah.blah", Some(dom_uuid)).await,
            "https://blah.blah:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/policy");
        assert_eq!(FmcApi::Deployment.path_string("a-b-c-d.local", Some(dom_uuid)).await,
            "https://a-b-c-d.local:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/deployment");
        assert_eq!(
            FmcApi::Updates
                .path_string("123.23.3.1", Some(dom_uuid))
                .await,
            "https://123.23.3.1:443/api/fmc_config/v1/domain/"
        );
        assert_eq!(FmcApi::Audit.path_string("voodooCat.co.uk", Some(dom_uuid)).await,
            "https://voodooCat.co.uk:443/api/fmc_platform/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/audit/auditrecords");
        assert_eq!(
            FmcApi::Info.path_string("doctor.who", Some(dom_uuid)).await,
            "https://doctor.who:443/api/fmc_platform/v1/info"
        );
        assert_eq!(FmcApi::TaxiiConfig.path_string("murica.usa", Some(dom_uuid)).await,
            "https://murica.usa:443/api/fmc_tid/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/taxiiconfig");
        assert_eq!(
            FmcApi::Tid.path_string("8.8.8.8", Some(dom_uuid)).await,
            "https://8.8.8.8:443/api/fmc_tid/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/tid"
        );

        // Inverse tests
        assert_ne!(
            FmcApi::Tid.path_string("8.8.8.7", Some(dom_uuid)).await,
            "https://8.8.8.8:443/api/fmc_tid/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/tid"
        );
        assert_ne!(FmcApi::Object.path_string("fmc-01.Sw.local", Some(dom_uuid)).await,
            "https://fmc-01.sw.local:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203040506/object");
        assert_ne!(FmcApi::Integration.path_string("10.0.0.5", Some(dom_uuid)).await,
            "https://10.0.0.5:443/api/fmc_config/v1/domain/f3b4958c-52a1-11e7-802a-010203041506/object/integration");
    }
}
