/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2023-08-08T23:00:01Z
 *
 * Generated by: https://openapi-generator.tech
 */

/// DomainPeriodDiscoverApiioTHost : Represents information about a managed, an unmanaged or an unsupported asset.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainPeriodDiscoverApiioTHost {
    /// The version of the Falcon sensor that's installed on the asset.
    #[serde(rename = "agent_version", skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// The agent ID of the Falcon sensor installed on the asset.
    #[serde(rename = "aid", skip_serializing_if = "Option::is_none")]
    pub aid: Option<String>,
    /// The Amount of available disk space on the asset in GB
    #[serde(
        rename = "available_disk_space",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_disk_space: Option<i32>,
    /// The average memory usage in the last 15 minutes on the asset
    #[serde(
        rename = "average_memory_usage",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_memory_usage: Option<i32>,
    /// The average processor usage in the last 15 minutes on the asset
    #[serde(
        rename = "average_processor_usage",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_processor_usage: Option<i32>,
    /// The id of the bios on the asset
    #[serde(rename = "bios_id", skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    /// The name of the asset's BIOS manufacturer.
    #[serde(rename = "bios_manufacturer", skip_serializing_if = "Option::is_none")]
    pub bios_manufacturer: Option<String>,
    /// The asset's BIOS version.
    #[serde(rename = "bios_version", skip_serializing_if = "Option::is_none")]
    pub bios_version: Option<String>,
    /// The business criticality of the IoT asset.
    #[serde(
        rename = "business_criticality",
        skip_serializing_if = "Option::is_none"
    )]
    pub business_criticality: Option<String>,
    /// The asset's customer ID.
    #[serde(rename = "cid")]
    pub cid: String,
    /// The name of the city where the asset is located.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The external ID of the IoT Device in 3rd Party System(Claroty).
    #[serde(rename = "claroty_id", skip_serializing_if = "Option::is_none")]
    pub claroty_id: Option<String>,
    /// The level of confidence that the asset is a corporate asset (25 = low confidence, 50 = medium confidence, 75 = high confidence).
    #[serde(rename = "confidence", skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i32>,
    /// The name of the country where the asset is located.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The Detailed processor name
    #[serde(rename = "cpu_processor_name", skip_serializing_if = "Option::is_none")]
    pub cpu_processor_name: Option<String>,
    /// The credential guard status of the asset
    #[serde(
        rename = "credential_guard_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_guard_status: Option<bool>,
    /// The last seen local IPv4 address of the asset.
    #[serde(rename = "current_local_ip", skip_serializing_if = "Option::is_none")]
    pub current_local_ip: Option<String>,
    /// The asset's data providers.
    #[serde(rename = "data_providers", skip_serializing_if = "Option::is_none")]
    pub data_providers: Option<Vec<String>>,
    /// The number of data providers for the asset.
    #[serde(
        rename = "data_providers_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_providers_count: Option<i32>,
    /// The Device Class of IoT Asset
    #[serde(rename = "device_class", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,
    /// The Device Family of IoT Asset
    #[serde(rename = "device_family", skip_serializing_if = "Option::is_none")]
    pub device_family: Option<String>,
    /// The device guard status of the asset
    #[serde(
        rename = "device_guard_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_guard_status: Option<bool>,
    /// The slots of IoT Asset
    #[serde(rename = "device_slots", skip_serializing_if = "Option::is_none")]
    pub device_slots: Option<Vec<crate::models::DomainPeriodDiscoverApiDeviceSlot>>,
    /// The Device Type of IoT Asset
    #[serde(rename = "device_type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// The number of sources that discovered the asset.
    #[serde(rename = "discoverer_count", skip_serializing_if = "Option::is_none")]
    pub discoverer_count: Option<i32>,
    /// A list of agent IDs of the Falcon sensors installed on the source hosts that discovered the asset via ICS Asset discovery mechanism
    #[serde(
        rename = "discoverer_ics_collector_ids",
        skip_serializing_if = "Option::is_none"
    )]
    pub discoverer_ics_collector_ids: Option<Vec<String>>,
    /// The product type descriptions of the sources that discovered the asset.
    #[serde(
        rename = "discoverer_product_type_descs",
        skip_serializing_if = "Option::is_none"
    )]
    pub discoverer_product_type_descs: Option<Vec<String>>,
    /// The names and sizes of the disks on the asset
    #[serde(rename = "disk_sizes", skip_serializing_if = "Option::is_none")]
    pub disk_sizes: Option<Vec<crate::models::DomainPeriodDiscoverApiDiskSize>>,
    /// The list of encrypted drives on the asset
    #[serde(rename = "encrypted_drives", skip_serializing_if = "Option::is_none")]
    pub encrypted_drives: Option<Vec<String>>,
    /// The count of encrypted drives on the asset
    #[serde(
        rename = "encrypted_drives_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub encrypted_drives_count: Option<i32>,
    /// The encryption status of the asset
    #[serde(rename = "encryption_status", skip_serializing_if = "Option::is_none")]
    pub encryption_status: Option<String>,
    /// The type of asset (managed, unmanaged, unsupported).
    #[serde(rename = "entity_type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// The external IPv4 address of the asset.
    #[serde(rename = "external_ip", skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    /// Lists the data providers for each property in the response (Cannot be used for filtering, sorting, or querying).
    #[serde(rename = "field_metadata", skip_serializing_if = "Option::is_none")]
    pub field_metadata: Option<
        ::std::collections::HashMap<String, crate::models::DomainPeriodDiscoverApiFieldMetadata>,
    >,
    /// The first time the asset was seen in your environment.
    #[serde(
        rename = "first_seen_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_seen_timestamp: Option<String>,
    /// The host management groups the asset is part of.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// The asset's hostname .
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The ID generated by ICS collector asset discovery mechanism
    #[serde(rename = "ics_id", skip_serializing_if = "Option::is_none")]
    pub ics_id: Option<String>,
    /// The unique ID of the asset.
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the asset is exposed to the internet (Yes or Unknown)
    #[serde(rename = "internet_exposure", skip_serializing_if = "Option::is_none")]
    pub internet_exposure: Option<String>,
    /// The iommu protection status of the host
    #[serde(
        rename = "iommu_protection_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub iommu_protection_status: Option<String>,
    /// The kernel dma protection status of the asset
    #[serde(
        rename = "kernel_dma_protection_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub kernel_dma_protection_status: Option<bool>,
    /// For Linux and Mac hosts: the major version, minor version, and patch version of the kernel for the asset. For Windows hosts: the build number of the asset.
    #[serde(rename = "kernel_version", skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    /// The agent ID of the Falcon sensor installed on the source host that most recently discovered the asset via ICS Asset discovery mechanism
    #[serde(
        rename = "last_discoverer_ics_collector_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_discoverer_ics_collector_id: Option<String>,
    /// The most recent time the asset was seen in your environment.
    #[serde(
        rename = "last_seen_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_seen_timestamp: Option<String>,
    /// The IoT asset's IP address list
    #[serde(rename = "local_ip_addresses", skip_serializing_if = "Option::is_none")]
    pub local_ip_addresses: Option<Vec<String>>,
    /// The number of historical local IPv4 addresses the asset has had.
    #[serde(rename = "local_ips_count", skip_serializing_if = "Option::is_none")]
    pub local_ips_count: Option<i32>,
    /// The Number of Logical Cores on the asset
    #[serde(rename = "logical_core_count", skip_serializing_if = "Option::is_none")]
    pub logical_core_count: Option<i32>,
    /// The IoT asset's MAC address list
    #[serde(rename = "mac_addresses", skip_serializing_if = "Option::is_none")]
    pub mac_addresses: Option<Vec<String>>,
    /// The domain name the asset is currently joined to (applies only to Windows hosts).
    #[serde(rename = "machine_domain", skip_serializing_if = "Option::is_none")]
    pub machine_domain: Option<String>,
    /// The max memory usage in the last 15 minutes on the asset
    #[serde(rename = "max_memory_usage", skip_serializing_if = "Option::is_none")]
    pub max_memory_usage: Option<i32>,
    /// The max processor usage in the last 15 minutes on the asset
    #[serde(
        rename = "max_processor_usage",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_processor_usage: Option<i32>,
    /// The Total memory.
    #[serde(rename = "memory_total", skip_serializing_if = "Option::is_none")]
    pub memory_total: Option<i64>,
    /// The path, used and available space on mounted disks
    #[serde(rename = "mount_storage_info", skip_serializing_if = "Option::is_none")]
    pub mount_storage_info: Option<Vec<crate::models::DomainPeriodDiscoverApiMountStorageInfo>>,
    /// The network ID to which device is connected.
    #[serde(rename = "network_id", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// The asset's network interfaces.
    #[serde(rename = "network_interfaces", skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<crate::models::DomainPeriodDiscoverApiNetworkInterface>>,
    /// The number of active physical drives available on the system
    #[serde(
        rename = "number_of_disk_drives",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_disk_drives: Option<i32>,
    /// Whether the asset is at end of support (Yes, No, or Unknown)
    #[serde(rename = "os_is_eol", skip_serializing_if = "Option::is_none")]
    pub os_is_eol: Option<String>,
    /// The OS version of the asset.
    #[serde(rename = "os_version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The organizational unit of the asset (applies only to Windows hosts).
    #[serde(rename = "ou", skip_serializing_if = "Option::is_none")]
    pub ou: Option<String>,
    /// The number of physical CPU cores available on the system
    #[serde(
        rename = "physical_core_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_core_count: Option<i32>,
    /// The platform name of the asset (Windows, Mac, Linux).
    #[serde(rename = "platform_name", skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    /// The number of physical processors available on the system
    #[serde(
        rename = "processor_package_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub processor_package_count: Option<i32>,
    /// The product type of the asset represented as a number (1 = Workstation, 2 = Domain Controller, 3 = Server).
    #[serde(rename = "product_type", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// The product type of the asset (Workstation, Domain Controller, Server).
    #[serde(rename = "product_type_desc", skip_serializing_if = "Option::is_none")]
    pub product_type_desc: Option<String>,
    /// The list of protocols supported by the device
    #[serde(rename = "protocols", skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    /// The purdue level of IoT Asset
    #[serde(rename = "purdue_level", skip_serializing_if = "Option::is_none")]
    pub purdue_level: Option<String>,
    /// Whether the asset is in reduced functionality mode (Yes or No)
    #[serde(
        rename = "reduced_functionality_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub reduced_functionality_mode: Option<String>,
    /// The secure boot enabled status of the asset
    #[serde(
        rename = "secure_boot_enabled_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub secure_boot_enabled_status: Option<bool>,
    /// The secure boot requested status of the asset
    #[serde(
        rename = "secure_boot_requested_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub secure_boot_requested_status: Option<bool>,
    /// The secure memory overwrite requested status of the asset
    #[serde(
        rename = "secure_memory_overwrite_requested_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub secure_memory_overwrite_requested_status: Option<String>,
    /// The site name of the domain the asset is joined to (applies only to Windows hosts).
    #[serde(rename = "site_name", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// The subnet to which device is connected.
    #[serde(rename = "subnet", skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    /// The system guard status of the asset
    #[serde(
        rename = "system_guard_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_guard_status: Option<String>,
    /// The asset's system manufacturer.
    #[serde(
        rename = "system_manufacturer",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_manufacturer: Option<String>,
    /// The asset's system product name.
    #[serde(
        rename = "system_product_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_product_name: Option<String>,
    /// The asset's system serial number.
    #[serde(
        rename = "system_serial_number",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_serial_number: Option<String>,
    /// The sensor and cloud tags of the asset.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The count of bios files measured by the firmware image
    #[serde(rename = "total_bios_files", skip_serializing_if = "Option::is_none")]
    pub total_bios_files: Option<i32>,
    /// The Total amount of disk space available on the asset in GB
    #[serde(rename = "total_disk_space", skip_serializing_if = "Option::is_none")]
    pub total_disk_space: Option<i32>,
    /// The uefi memory protection status of the asset
    #[serde(
        rename = "uefi_memory_protection_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub uefi_memory_protection_status: Option<String>,
    /// The list of unencrypted drives on the asset
    #[serde(rename = "unencrypted_drives", skip_serializing_if = "Option::is_none")]
    pub unencrypted_drives: Option<Vec<String>>,
    /// The count of unencrypted drives on the asset
    #[serde(
        rename = "unencrypted_drives_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub unencrypted_drives_count: Option<i32>,
    /// The Current amount of used disk space on the asset in GB
    #[serde(rename = "used_disk_space", skip_serializing_if = "Option::is_none")]
    pub used_disk_space: Option<i32>,
    /// The Virtual Zone name in which device is installed.
    #[serde(rename = "virtual_zone", skip_serializing_if = "Option::is_none")]
    pub virtual_zone: Option<String>,
    /// The virtualization based security status of the asset
    #[serde(
        rename = "virtualization_based_security_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtualization_based_security_status: Option<bool>,
    /// The VLAN IDs to which device is connected.
    #[serde(rename = "vlan", skip_serializing_if = "Option::is_none")]
    pub vlan: Option<Vec<String>>,
    /// The external ID of the IoT Device in 3rd Party System(Claroty Xdome)
    #[serde(rename = "xdome_id", skip_serializing_if = "Option::is_none")]
    pub xdome_id: Option<String>,
}

impl DomainPeriodDiscoverApiioTHost {
    /// Represents information about a managed, an unmanaged or an unsupported asset.
    pub fn new(cid: String, id: String) -> DomainPeriodDiscoverApiioTHost {
        DomainPeriodDiscoverApiioTHost {
            agent_version: None,
            aid: None,
            available_disk_space: None,
            average_memory_usage: None,
            average_processor_usage: None,
            bios_id: None,
            bios_manufacturer: None,
            bios_version: None,
            business_criticality: None,
            cid,
            city: None,
            claroty_id: None,
            confidence: None,
            country: None,
            cpu_processor_name: None,
            credential_guard_status: None,
            current_local_ip: None,
            data_providers: None,
            data_providers_count: None,
            device_class: None,
            device_family: None,
            device_guard_status: None,
            device_slots: None,
            device_type: None,
            discoverer_count: None,
            discoverer_ics_collector_ids: None,
            discoverer_product_type_descs: None,
            disk_sizes: None,
            encrypted_drives: None,
            encrypted_drives_count: None,
            encryption_status: None,
            entity_type: None,
            external_ip: None,
            field_metadata: None,
            first_seen_timestamp: None,
            groups: None,
            hostname: None,
            ics_id: None,
            id,
            internet_exposure: None,
            iommu_protection_status: None,
            kernel_dma_protection_status: None,
            kernel_version: None,
            last_discoverer_ics_collector_id: None,
            last_seen_timestamp: None,
            local_ip_addresses: None,
            local_ips_count: None,
            logical_core_count: None,
            mac_addresses: None,
            machine_domain: None,
            max_memory_usage: None,
            max_processor_usage: None,
            memory_total: None,
            mount_storage_info: None,
            network_id: None,
            network_interfaces: None,
            number_of_disk_drives: None,
            os_is_eol: None,
            os_version: None,
            ou: None,
            physical_core_count: None,
            platform_name: None,
            processor_package_count: None,
            product_type: None,
            product_type_desc: None,
            protocols: None,
            purdue_level: None,
            reduced_functionality_mode: None,
            secure_boot_enabled_status: None,
            secure_boot_requested_status: None,
            secure_memory_overwrite_requested_status: None,
            site_name: None,
            subnet: None,
            system_guard_status: None,
            system_manufacturer: None,
            system_product_name: None,
            system_serial_number: None,
            tags: None,
            total_bios_files: None,
            total_disk_space: None,
            uefi_memory_protection_status: None,
            unencrypted_drives: None,
            unencrypted_drives_count: None,
            used_disk_space: None,
            virtual_zone: None,
            virtualization_based_security_status: None,
            vlan: None,
            xdome_id: None,
        }
    }
}