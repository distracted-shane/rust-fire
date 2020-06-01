use serde::Deserialize;

// Core JSON blocks that are used in many places
mod core {
    use super::*;

    #[derive(Deserialize, Debug)]
    pub(super) struct Domain {
        name: String,
        id: String,
        uuid: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct Ipv6 {
        #[serde(alias = "enforeEUI64")]
        enforce_eui64: bool,
        #[serde(alias = "enforceAutoConfig")]
        enable_auto_config: bool,
        #[serde(alias = "enableDHCPAddrConfig")]
        enable_dhcp_addr_config: bool,
        #[serde(alias = "enableDHCPNonAddrConfig")]
        enable_dhcp_nonaddr_config: bool,
        #[serde(alias = "dadAttempts")]
        dad_attempts: u32,
        #[serde(alias = "nsInterval")]
        ns_interval: u32,
        #[serde(alias = "reachableTime")]
        reachable_time: u32,
        #[serde(alias = "enableRA")]
        enable_ra: bool,
        #[serde(alias = "raLifeTime")]
        ra_lifetime: u32,
        #[serde(alias = "raInterval")]
        ra_interval: u32,
        #[serde(alias = "enableIPV6")]
        enable_ipv6: bool,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct Links {
        #[serde(alias = "self")]
        _self: String,
        parent: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct LastUser {
        name: String,
        id: Option<String>,
        #[serde(alias = "type")]
        _type: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct MetaData {
        #[serde(alias = "readOnly")]
        read_only: Option<ReadOnly>,
        #[serde(alias = "lastUser")]
        last_user: Option<LastUser>,
        domain: Option<Domain>,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct Paging {
        offset: u16,
        limit: u16,
        count: u16,
        pages: u16,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct ReadOnly {
        state: bool,
        reason: String,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct SecurityZone {
        id: String,
        #[serde(alias = "type")]
        _type: String,
    }
}

mod devices {
    use super::*;

    // /api/fmc_config/v1/domain/{domainUUID}/devices/devicerecords
    #[derive(Deserialize, Debug)]
    pub(super) struct DeviceRecords {
        links: core::Links,
        items: Vec<DeviceItem>,
        paging: core::Paging,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct DeviceItem {
        id: String,
        #[serde(alias = "type")]
        _type: String,
        links: core::Links,
        name: String,
        #[serde(alias = "hostName")]
        host_name: String,
        #[serde(alias = "ftdMode")]
        ftd_mode: String,
    }

    // /api/fmc_config/v1/domain/{domainUUID}/devices/devicerecords/{containerUUID}/physicalinterfaces
    #[derive(Deserialize, Debug)]
    pub(super) struct PhysicalInterfaces {
        links: core::Links,
        items: Vec<PhysIntItem>,
        paging: core::Paging,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct PhysIntItem {
        links: core::Links,
        name: String,
        id: String,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct EtherChannelInts {
        links: core::Links,
        items: Vec<EtherChannelInt>,
        paging: core::Paging,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct EtherChannelInt {
        links: core::Links,
        name: String,
        id: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // GET // /api/fmc_config/v1/domain/{domainUUID}/devices/devicerecords
    #[test]
    fn device_records_test() {
        let raw_string: &str = r#"{
            "links": {
              "self": "https://10.17.11.151/api/fmc_config/v1/domain/e276abec-e0f2-11e3-8169-6d9ed49b625f/devices/devicerecords?offset=0&limit=2&expanded=true"
            },
            "items": [
              {
                "id": "966e41da-a0f7-11ea-b9d2-a72388220492",
                "type": "Device",
                "links": {
                  "self": "https://10.17.11.151/api/fmc_config/v1/domain/e276abec-e0f2-11e3-8169-6d9ed49b625f/devices/devicerecords/966e41da-a0f7-11ea-b9d2-a72388220492"
                },
                "name": "Cat",
                "description": "NOT SUPPORTED",
                "model": "Cisco Firepower Threat Defense for VMWare",
                "modelId": "A",
                "modelNumber": "75",
                "modelType": "Sensor",
                "healthStatus": "green",
                "sw_version": "6.5.0",
                "healthPolicy": {
                  "id": "b352b258-a102-11ea-b0fe-b993706bf961",
                  "type": "HealthPolicy",
                  "name": "Initial_Health_Policy 2020-05-28 16:45:55"
                },
                "accessPolicy": {
                  "name": "Basic IPS",
                  "id": "000c290c-de43-0ed3-0000-004294967320",
                  "type": "AccessPolicy"
                },
                "hostName": "10.17.11.155",
                "license_caps": [
                  "BASE"
                ],
                "keepLocalEvents": false,
                "prohibitPacketTransfer": true,
                "ftdMode": "ROUTED",
                "metadata": {
                  "readOnly": {
                    "state": false
                  },
                  "inventoryData": {
                    "cpuCores": "1 CPU (4 cores)",
                    "cpuType": "CPU Lynnfield 3408 MHz",
                    "memoryInMB": "8192"
                  },
                  "domain": {
                    "name": "Global",
                    "id": "e276abec-e0f2-11e3-8169-6d9ed49b625f",
                    "type": "Domain"
                  },
                  "isPartOfContainer": false,
                  "isMultiInstance": false
                }
              },
              {
                "id": "09e79498-a0fa-11ea-8057-b3ca2b41109a",
                "type": "Device",
                "links": {
                  "self": "https://10.17.11.151/api/fmc_config/v1/domain/e276abec-e0f2-11e3-8169-6d9ed49b625f/devices/devicerecords/09e79498-a0fa-11ea-8057-b3ca2b41109a"
                },
                "name": "Poop",
                "description": "NOT SUPPORTED",
                "model": "Cisco Firepower Threat Defense for VMWare",
                "modelId": "A",
                "modelNumber": "75",
                "modelType": "Sensor",
                "healthStatus": "green",
                "sw_version": "6.5.0",
                "healthPolicy": {
                  "id": "b352b258-a102-11ea-b0fe-b993706bf961",
                  "type": "HealthPolicy",
                  "name": "Initial_Health_Policy 2020-05-28 16:45:55"
                },
                "accessPolicy": {
                  "name": "Basic IPS",
                  "id": "000c290c-de43-0ed3-0000-004294967320",
                  "type": "AccessPolicy"
                },
                "hostName": "10.17.11.156",
                "license_caps": [
                  "BASE"
                ],
                "keepLocalEvents": false,
                "prohibitPacketTransfer": true,
                "ftdMode": "ROUTED",
                "metadata": {
                  "readOnly": {
                    "state": false
                  },
                  "inventoryData": {
                    "cpuCores": "1 CPU (4 cores)",
                    "cpuType": "CPU Lynnfield 3408 MHz",
                    "memoryInMB": "8192"
                  },
                  "domain": {
                    "name": "Global",
                    "id": "e276abec-e0f2-11e3-8169-6d9ed49b625f",
                    "type": "Domain"
                  },
                  "isPartOfContainer": false,
                  "isMultiInstance": false
                }
              }
            ],
            "paging": {
              "offset": 0,
              "limit": 2,
              "count": 2,
              "pages": 1
            }
          }

          "#;

        let parsed: devices::DeviceRecords = serde_json::from_str(raw_string).unwrap();
        println!("{:?}\n\n", parsed);
    }

    // GET /api/fmc_config/v1/domain/{domainUUID}/devices/devicerecords/{containerUUID}/physicalinterfaces
    #[test]
    fn device_phys_int_test() {
        let raw_string = r#"
        {
            "links": {
                "self": "/fmc_config/v1/domain/default/devices/devicerecords/deviceUUID/physicalinterfaces?offset=0&limit=4"
            },
            "items": [
                {
                    "links": {
                        "self": "/fmc_config/v1/domain/default/devices/devicerecords/deviceUUID/physicalinterfaces/PhyIntfId1"
                    },
                    "name": "GigabitEthernet1/1",
                    "id": "PhyIntfId1"
                },
                {
                    "links": {
                        "self": "/fmc_config/v1/domain/default/devices/devicerecords/deviceUUID/physicalinterfaces/PhyIntfId2"
                    },
                    "name": "GigabitEthernet1/2",
                    "id": "PhyIntfId2"
                },
                {
                    "links": {
                        "self": "/fmc_config/v1/domain/default/devices/devicerecords/deviceUUID/physicalinterfaces/PhyIntfId3"
                    },
                    "name": "GigabitEthernet1/3",
                    "id": "PhyIntfId3"
                },
                {
                    "links": {
                        "self": "/fmc_config/v1/domain/default/devices/devicerecords/deviceUUID/physicalinterfaces/PhyIntfId4"
                    },
                    "name": "GigabitEthernet1/4",
                    "id": "PhyIntfId4"
                }
            ],
            "paging": {
                "offset": 0,
                "limit": 4,
                "count": 4,
                "pages": 1
            }
        }
        "#;
        let parsed: devices::PhysicalInterfaces = serde_json::from_str(raw_string).unwrap();
        println!("{:?}\n\n", parsed);
    }

    // GET /fmc_config/v1/domain/DomainUUID/devices/devicerecords/containerUUID/etherchannelinterfaces
    #[test]
    fn device_etherch_int_test() {
        let raw_str = r#"{
            "links": {
                "self": "/fmc_config/v1/domain/default/devices/devicerecords/containerUUID/etherchannelinterfaces?offset=0&limit=2"
            },
            "items": [
                {
                    "links": {
                        "self": "/fmc_config/v1/domain/default/devices/devicerecords/containerUUID/etherchannelinterfaces/etherChannelIntfUUID2"
                    },
                    "name": "Port-channel2",
                    "id": "etherChannelIntfUUID2"
                },
                {
                    "links": {
                        "self": "/fmc_config/v1/domain/default/devices/devicerecords/containerUUID/etherchannelinterfaces/etherChannelIntfUUID1"
                    },
                    "name": "Port-channel1",
                    "id": "etherChannelIntfUUID1"
                }
            ],
            "paging": {
                "offset": 0,
                "limit": 2,
                "count": 2,
                "pages": 1
            }
        }"#;
        let parsed: devices::EtherChannelInts = serde_json::from_str(raw_str).unwrap();
        println!("{:?}\n\n", parsed);
    }
}
