/// Traffic density indicator on a contiguous segment of a path. Given a path
/// with points P_0, P_1, ... , P_N (zero-based index), the SpeedReadingInterval
/// defines an interval and describes its traffic using the following categories.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeedReadingInterval {
    /// The starting index of this interval in the path.
    /// In JSON, when the index is 0, the field will appear to be unpopulated.
    #[prost(int32, tag="1")]
    pub start_polyline_point_index: i32,
    /// The ending index of this interval in the path.
    /// In JSON, when the index is 0, the field will appear to be unpopulated.
    #[prost(int32, tag="2")]
    pub end_polyline_point_index: i32,
    /// Traffic speed in this interval.
    #[prost(enumeration="speed_reading_interval::Speed", tag="3")]
    pub speed: i32,
}
/// Nested message and enum types in `SpeedReadingInterval`.
pub mod speed_reading_interval {
    /// The classification of polyline speed based on traffic data.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Speed {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Normal speed, no slowdown is detected.
        Normal = 1,
        /// Slowdown detected, but no traffic jam formed.
        Slow = 2,
        /// Traffic jam detected.
        TrafficJam = 3,
    }
}
/// Traffic density along a Vehicle's path.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumableTrafficPolyline {
    /// Traffic speed along the path from the previous waypoint to the current
    /// waypoint.
    #[prost(message, repeated, tag="1")]
    pub speed_reading_interval: ::prost::alloc::vec::Vec<SpeedReadingInterval>,
    /// The path the driver is taking from the previous waypoint to the current
    /// waypoint. This path has landmarks in it so clients can show traffic markers
    /// along the path (see `speed_reading_interval`).  Decoding is not yet
    /// supported.
    #[prost(string, tag="2")]
    pub encoded_path_to_waypoint: ::prost::alloc::string::String,
}
/// Identifies a terminal point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerminalPointId {
    /// Unique ID of the terminal point.
    #[prost(string, tag="4")]
    pub value: ::prost::alloc::string::String,
    /// Deprecated.
    #[prost(oneof="terminal_point_id::Id", tags="2, 3")]
    pub id: ::core::option::Option<terminal_point_id::Id>,
}
/// Nested message and enum types in `TerminalPointId`.
pub mod terminal_point_id {
    /// Deprecated.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        /// Deprecated.
        #[prost(string, tag="2")]
        PlaceId(::prost::alloc::string::String),
        /// Deprecated.
        #[prost(string, tag="3")]
        GeneratedId(::prost::alloc::string::String),
    }
}
/// Describes the location of a waypoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerminalLocation {
    /// Required. Denotes the location of a trip waypoint.
    #[prost(message, optional, tag="1")]
    pub point: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// ID of the terminal point.
    #[prost(message, optional, tag="2")]
    pub terminal_point_id: ::core::option::Option<TerminalPointId>,
    /// Deprecated.
    #[deprecated]
    #[prost(string, tag="3")]
    pub access_point_id: ::prost::alloc::string::String,
    /// Deprecated.
    #[deprecated]
    #[prost(string, tag="4")]
    pub trip_id: ::prost::alloc::string::String,
    /// Deprecated: `Vehicle.waypoint` will have this data.
    #[deprecated]
    #[prost(enumeration="WaypointType", tag="5")]
    pub terminal_location_type: i32,
}
/// Describes a stopping point on a vehicle's route or an ending point on a
/// vehicle's trip.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripWaypoint {
    /// The location of this waypoint.
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<TerminalLocation>,
    /// The trip associated with this waypoint.
    #[prost(string, tag="2")]
    pub trip_id: ::prost::alloc::string::String,
    /// The role this waypoint plays in this trip, such as pickup or dropoff.
    #[prost(enumeration="WaypointType", tag="3")]
    pub waypoint_type: i32,
    /// The path from the previous waypoint to the current waypoint.  Undefined for
    /// the first waypoint in a list. This field is only populated when requested.
    #[prost(message, repeated, tag="4")]
    pub path_to_waypoint: ::prost::alloc::vec::Vec<super::super::super::google::r#type::LatLng>,
    /// The encoded path from the previous waypoint to the current waypoint.
    ///
    /// <p>Note: This field is intended only for use by the Driver SDK and Consumer
    /// SDK. Decoding is not yet supported.
    #[prost(string, tag="5")]
    pub encoded_path_to_waypoint: ::prost::alloc::string::String,
    /// The traffic conditions along the path to this waypoint.  Note that traffic
    /// is only available for Google Map Platform Rides and Deliveries Solution
    /// customers.
    #[prost(message, optional, tag="10")]
    pub traffic_to_waypoint: ::core::option::Option<ConsumableTrafficPolyline>,
    /// The path distance from the previous waypoint to the current waypoint.
    /// Undefined for the first waypoint in a list.
    #[prost(message, optional, tag="6")]
    pub distance_meters: ::core::option::Option<i32>,
    /// The estimated time of arrival at this waypoint. Undefined for the first
    /// waypoint in a list.
    #[prost(message, optional, tag="7")]
    pub eta: ::core::option::Option<::prost_types::Timestamp>,
    /// The travel time from previous waypoint to this point. Undefined for the
    /// first waypoint in a list.
    #[prost(message, optional, tag="8")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
/// Describes a vehicle attribute as a key-value pair. The "key:value" string
/// length cannot exceed 256 characters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleAttribute {
    /// The attribute's key. Keys may not contain the colon character (:).
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    /// The attribute's value.
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
/// The location, speed, and heading of a vehicle at a point in time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleLocation {
    /// The location of the vehicle.
    /// When it is sent to Fleet Engine, the vehicle's location is a GPS location.
    /// When you receive it in a response, the vehicle's location can be either a
    /// GPS location, a supplemental location, or some other estimated location.
    /// The source is specified in `location_sensor`.
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// Deprecated: Use `latlng_accuracy` instead.
    #[deprecated]
    #[prost(message, optional, tag="8")]
    pub horizontal_accuracy: ::core::option::Option<f64>,
    /// Accuracy of `location` in meters as a radius.
    #[prost(message, optional, tag="22")]
    pub latlng_accuracy: ::core::option::Option<f64>,
    /// Direction the vehicle is moving in degrees.  0 represents North.
    /// The valid range is [0,360).
    #[prost(message, optional, tag="2")]
    pub heading: ::core::option::Option<i32>,
    /// Deprecated: Use `heading_accuracy` instead.
    #[deprecated]
    #[prost(message, optional, tag="10")]
    pub bearing_accuracy: ::core::option::Option<f64>,
    /// Accuracy of `heading` in degrees.
    #[prost(message, optional, tag="23")]
    pub heading_accuracy: ::core::option::Option<f64>,
    /// Altitude in meters above WGS84.
    #[prost(message, optional, tag="5")]
    pub altitude: ::core::option::Option<f64>,
    /// Deprecated: Use `altitude_accuracy` instead.
    #[deprecated]
    #[prost(message, optional, tag="9")]
    pub vertical_accuracy: ::core::option::Option<f64>,
    /// Accuracy of `altitude` in meters.
    #[prost(message, optional, tag="24")]
    pub altitude_accuracy: ::core::option::Option<f64>,
    /// Speed of the vehicle in kilometers per hour.
    /// Deprecated: Use `speed` instead.
    #[deprecated]
    #[prost(message, optional, tag="3")]
    pub speed_kmph: ::core::option::Option<i32>,
    /// Speed of the vehicle in meters/second
    #[prost(message, optional, tag="6")]
    pub speed: ::core::option::Option<f64>,
    /// Accuracy of `speed` in meters/second.
    #[prost(message, optional, tag="7")]
    pub speed_accuracy: ::core::option::Option<f64>,
    /// The time when `location` was reported by the sensor according to the
    /// sensor's clock.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the server received the location information.
    #[prost(message, optional, tag="13")]
    pub server_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Provider of location data (for example, `GPS`).
    #[prost(enumeration="LocationSensor", tag="11")]
    pub location_sensor: i32,
    /// Whether `location` is snapped to a road.
    #[prost(message, optional, tag="27")]
    pub is_road_snapped: ::core::option::Option<bool>,
    /// Input only. Indicates whether the GPS sensor is enabled on the mobile device.
    #[prost(message, optional, tag="12")]
    pub is_gps_sensor_enabled: ::core::option::Option<bool>,
    /// Input only. Time (in seconds) since this location was first sent to the server.
    /// This will be zero for the first update. If the time is unknown
    /// (for example, when the app restarts), this value resets to zero.
    #[prost(message, optional, tag="14")]
    pub time_since_update: ::core::option::Option<i32>,
    /// Input only. Number of additional attempts to send this location to the server.
    /// If this value is zero, then it is not stale.
    #[prost(message, optional, tag="15")]
    pub num_stale_updates: ::core::option::Option<i32>,
    /// Raw vehicle location (unprocessed by road-snapper).
    #[prost(message, optional, tag="16")]
    pub raw_location: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// Input only. Timestamp associated with the raw location.
    #[prost(message, optional, tag="17")]
    pub raw_location_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Input only. Source of the raw location.
    #[prost(enumeration="LocationSensor", tag="28")]
    pub raw_location_sensor: i32,
    /// Input only. Accuracy of `raw_location` as a radius, in meters.
    #[prost(message, optional, tag="25")]
    pub raw_location_accuracy: ::core::option::Option<f64>,
    /// Input only. Supplemental location provided by the integrating app.
    #[prost(message, optional, tag="18")]
    pub supplemental_location: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// Input only. Timestamp associated with the supplemental location.
    #[prost(message, optional, tag="19")]
    pub supplemental_location_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Input only. Source of the supplemental location.
    #[prost(enumeration="LocationSensor", tag="20")]
    pub supplemental_location_sensor: i32,
    /// Input only. Accuracy of `supplemental_location` as a radius, in meters.
    #[prost(message, optional, tag="21")]
    pub supplemental_location_accuracy: ::core::option::Option<f64>,
    /// Deprecated: Use `is_road_snapped` instead.
    #[deprecated]
    #[prost(bool, tag="26")]
    pub road_snapped: bool,
}
/// The type of a trip.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TripType {
    /// Default, used for unspecified or unrecognized trip types.
    UnknownTripType = 0,
    /// The trip may share a vehicle with other trips.
    Shared = 1,
    /// The trip is exclusive to a vehicle.
    Exclusive = 2,
}
/// The type of waypoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WaypointType {
    /// Unknown or unspecified waypoint type.
    UnknownWaypointType = 0,
    /// Waypoints for picking up riders or items.
    PickupWaypointType = 1,
    /// Waypoints for dropping off riders or items.
    DropOffWaypointType = 2,
    /// Waypoints for intermediate destinations in a multi-destination trip.
    IntermediateDestinationWaypointType = 3,
}
/// The type of polyline format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolylineFormatType {
    /// The format is unspecified or unknown.
    UnknownFormatType = 0,
    /// A list of `google.type.LatLng`.
    LatLngListType = 1,
    /// A polyline encoded with a polyline compression algorithm. Decoding is not
    /// yet supported.
    EncodedPolylineType = 2,
}
/// The vehicle's navigation status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NavigationStatus {
    /// Unspecified navigation status.
    UnknownNavigationStatus = 0,
    /// The Driver app's navigation is in `FREE_NAV` mode.
    NoGuidance = 1,
    /// Turn-by-turn navigation is available and the Driver app navigation has
    /// entered `GUIDED_NAV` mode.
    EnrouteToDestination = 2,
    /// The vehicle has gone off the suggested route.
    OffRoute = 3,
    /// The vehicle is within approximately 50m of the destination.
    ArrivedAtDestination = 4,
}
/// The sensor or methodology used to determine the location.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocationSensor {
    /// The sensor is unspecified or unknown.
    UnknownSensor = 0,
    /// GPS or Assisted GPS.
    Gps = 1,
    /// Assisted GPS, cell tower ID, or WiFi access point.
    Network = 2,
    /// Cell tower ID or WiFi access point.
    Passive = 3,
    /// A location signal snapped to the best road position.
    RoadSnappedLocationProvider = 4,
    /// The fused location provider in Google Play services.
    FusedLocationProvider = 100,
}
/// Vehicle metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vehicle {
    /// Output only. The unique name for this vehicle.
    /// The format is `providers/{provider}/vehicles/{vehicle}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The vehicle state.
    #[prost(enumeration="VehicleState", tag="2")]
    pub vehicle_state: i32,
    /// Trip types supported by this vehicle.
    #[prost(enumeration="TripType", repeated, tag="3")]
    pub supported_trip_types: ::prost::alloc::vec::Vec<i32>,
    /// Output only. List of `trip_id`'s for trips currently assigned to this vehicle.
    #[prost(string, repeated, tag="4")]
    pub current_trips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Last reported location of the vehicle.
    #[prost(message, optional, tag="5")]
    pub last_location: ::core::option::Option<VehicleLocation>,
    /// The total numbers of riders this vehicle can carry.  The driver is not
    /// considered in this value. This value must be greater than or equal to one.
    #[prost(int32, tag="6")]
    pub maximum_capacity: i32,
    /// List of vehicle attributes. A vehicle can have at most 50
    /// attributes, and each attribute must have a unique key.
    #[prost(message, repeated, tag="8")]
    pub attributes: ::prost::alloc::vec::Vec<VehicleAttribute>,
    /// The type of this vehicle.  Can be used to filter vehicles in
    /// `SearchVehicles` results.  Also influences ETA and route calculations.
    #[prost(message, optional, tag="9")]
    pub vehicle_type: ::core::option::Option<vehicle::VehicleType>,
    /// License plate information for the vehicle.
    #[prost(message, optional, tag="10")]
    pub license_plate: ::core::option::Option<LicensePlate>,
    /// Deprecated: Use `Vehicle.waypoints` instead.
    #[deprecated]
    #[prost(message, repeated, tag="12")]
    pub route: ::prost::alloc::vec::Vec<TerminalLocation>,
    /// The polyline specifying the route the driver app intends to take to
    /// the next waypoint. This list is also returned in
    /// `Trip.current_route_segment` for all active trips assigned to the vehicle.
    ///
    /// Note: This field is intended only for use by the Driver SDK. Decoding is
    /// not yet supported.
    #[prost(string, tag="20")]
    pub current_route_segment: ::prost::alloc::string::String,
    /// Input only. Fleet Engine uses this information to improve Journey Sharing.
    #[prost(message, optional, tag="28")]
    pub current_route_segment_traffic: ::core::option::Option<TrafficPolylineData>,
    /// Output only. Time when `current_route_segment` was set. It can be stored by the client
    /// and passed in future `GetVehicle` requests to prevent returning routes that
    /// haven't changed.
    #[prost(message, optional, tag="15")]
    pub current_route_segment_version: ::core::option::Option<::prost_types::Timestamp>,
    /// The waypoint where `current_route_segment` ends. This can be supplied by
    /// drivers on `UpdateVehicle` calls either as a full trip waypoint, a waypoint
    /// `LatLng`, or as the last `LatLng` of the `current_route_segment`. Fleet
    /// Engine will then do its best to interpolate to an actual waypoint if it is
    /// not fully specified. This field is ignored in `UpdateVehicle` calls unless
    /// `current_route_segment` is also specified.
    #[prost(message, optional, tag="24")]
    pub current_route_segment_end_point: ::core::option::Option<TripWaypoint>,
    /// The remaining driving distance for the `current_route_segment`.
    /// This value is also returned in `Trip.remaining_distance_meters` for all
    /// active trips assigned to the vehicle. The value is unspecified if the
    /// `current_route_segment` field is empty.
    #[prost(message, optional, tag="18")]
    pub remaining_distance_meters: ::core::option::Option<i32>,
    /// The ETA to the first entry in the `waypoints` field.  The value is
    /// unspecified if the `waypoints` field is empty or the
    /// `Vehicle.current_route_segment` field is empty.
    ///
    /// When updating a vehicle, `remaining_time_seconds` takes precedence over
    /// `eta_to_first_waypoint` in the same request.
    #[prost(message, optional, tag="19")]
    pub eta_to_first_waypoint: ::core::option::Option<::prost_types::Timestamp>,
    /// Input only. The remaining driving time for the `current_route_segment`. The value is
    /// unspecified if the `waypoints` field is empty or the
    /// `Vehicle.current_route_segment` field is empty. This value should match
    /// `eta_to_first_waypoint` - `current_time` if all parties are using the same
    /// clock.
    ///
    /// When updating a vehicle, `remaining_time_seconds` takes precedence over
    /// `eta_to_first_waypoint` in the same request.
    #[prost(message, optional, tag="25")]
    pub remaining_time_seconds: ::core::option::Option<i32>,
    /// The remaining waypoints assigned to this Vehicle.
    #[prost(message, repeated, tag="22")]
    pub waypoints: ::prost::alloc::vec::Vec<TripWaypoint>,
    /// Output only. Last time the `waypoints` field was updated. Clients should cache this
    /// value and pass it in `GetVehicleRequest` to ensure the `waypoints` field is
    /// only returned if it is updated.
    #[prost(message, optional, tag="16")]
    pub waypoints_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates if the driver accepts back-to-back trips. If `true`,
    /// `SearchVehicles` may include the vehicle even if it is currently assigned
    /// to a trip. The default value is `false`.
    #[prost(bool, tag="23")]
    pub back_to_back_enabled: bool,
    /// The vehicle's navigation status.
    #[prost(enumeration="NavigationStatus", tag="26")]
    pub navigation_status: i32,
    /// Input only. Information about settings in the mobile device being used by the driver.
    #[prost(message, optional, tag="27")]
    pub device_settings: ::core::option::Option<DeviceSettings>,
}
/// Nested message and enum types in `Vehicle`.
pub mod vehicle {
    /// The type of vehicle.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VehicleType {
        /// Vehicle type category
        #[prost(enumeration="vehicle_type::Category", tag="1")]
        pub category: i32,
    }
    /// Nested message and enum types in `VehicleType`.
    pub mod vehicle_type {
        /// Vehicle type categories
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Category {
            /// Default, used for unspecified or unrecognized vehicle categories.
            Unknown = 0,
            /// An automobile.
            Auto = 1,
            /// Any vehicle that acts as a taxi (typically licensed or regulated).
            Taxi = 2,
            /// Generally, a vehicle with a large storage capacity.
            Truck = 3,
            /// A motorcycle, moped, or other two-wheeled vehicle
            TwoWheeler = 4,
        }
    }
}
/// Information about the device's battery.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatteryInfo {
    /// Status of the battery, whether full or charging etc.
    #[prost(enumeration="BatteryStatus", tag="1")]
    pub battery_status: i32,
    /// Status of battery power source.
    #[prost(enumeration="PowerSource", tag="2")]
    pub power_source: i32,
    /// Current battery percentage \[0-100\].
    #[prost(float, tag="3")]
    pub battery_percentage: f32,
}
/// Information about various settings on the mobile device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceSettings {
    /// How location features are set to behave on the device when battery saver is
    /// on.
    #[prost(enumeration="LocationPowerSaveMode", tag="1")]
    pub location_power_save_mode: i32,
    /// Whether the device is currently in power save mode.
    #[prost(bool, tag="2")]
    pub is_power_save_mode: bool,
    /// Whether the device is in an interactive state.
    #[prost(bool, tag="3")]
    pub is_interactive: bool,
    /// Information about the battery state.
    #[prost(message, optional, tag="4")]
    pub battery_info: ::core::option::Option<BatteryInfo>,
}
/// The license plate information of the Vehicle.  To avoid storing
/// personally-identifiable information, only the minimum information
/// about the license plate is stored as part of the entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LicensePlate {
    /// Required. CLDR Country/Region Code.  For example, `US` for United States,
    /// or `IN` for India.
    #[prost(string, tag="1")]
    pub country_code: ::prost::alloc::string::String,
    /// The last digit of the license plate or "-1" to denote no numeric value
    /// is present in the license plate.
    ///
    /// * "ABC 1234" -> "4"
    /// * "AB 123 CD" -> "3"
    /// * "ABCDEF" -> "-1"
    #[prost(string, tag="2")]
    pub last_character: ::prost::alloc::string::String,
}
/// Describes how clients should color one portion of the polyline along the
/// route.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisualTrafficReportPolylineRendering {
    /// Optional. Road stretches that should be rendered along the polyline. Stretches
    /// <ul>
    /// <li>are
    /// guaranteed to not overlap, and</li>
    /// <li>do not necessarily
    /// span the full route.</li>
    /// </ul>
    ///
    /// <p>In the absence of a road stretch to style, the client should apply the
    /// default for the route.
    #[prost(message, repeated, tag="1")]
    pub road_stretch: ::prost::alloc::vec::Vec<visual_traffic_report_polyline_rendering::RoadStretch>,
}
/// Nested message and enum types in `VisualTrafficReportPolylineRendering`.
pub mod visual_traffic_report_polyline_rendering {
    /// One road stretch that should be rendered.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoadStretch {
        /// Required. The style to apply.
        #[prost(enumeration="road_stretch::Style", tag="1")]
        pub style: i32,
        /// Required. The style should be applied between `[offset_meters, offset_meters +
        /// length_meters)`.
        #[prost(int32, tag="2")]
        pub offset_meters: i32,
        /// Required. The length of the path where to apply the style.
        #[prost(int32, tag="3")]
        pub length_meters: i32,
    }
    /// Nested message and enum types in `RoadStretch`.
    pub mod road_stretch {
        /// The traffic style, indicating traffic speed.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Style {
            /// No style selected.
            Unspecified = 0,
            /// Traffic is slowing down.
            SlowerTraffic = 1,
            /// There is a traffic jam.
            TrafficJam = 2,
        }
    }
}
/// Traffic conditions along the expected vehicle route.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficPolylineData {
    /// A polyline rendering of how fast traffic is for all regions along
    /// one stretch of a customer ride.
    #[prost(message, optional, tag="1")]
    pub traffic_rendering: ::core::option::Option<VisualTrafficReportPolylineRendering>,
}
/// The state of a `Vehicle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VehicleState {
    /// Default, used for unspecified or unrecognized vehicle states.
    UnknownVehicleState = 0,
    /// The vehicle is not accepting new trips. Note: the vehicle may continue to
    /// operate in this state while completing a trip assigned to it.
    Offline = 1,
    /// The vehicle is accepting new trips.
    Online = 2,
}
/// How location features are configured to behave on the mobile device when the
/// devices "battery saver" feature is on.
/// (<https://developer.android.com/reference/android/os/PowerManager#getLocationPowerSaveMode(>))
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocationPowerSaveMode {
    /// Undefined LocationPowerSaveMode
    UnknownLocationPowerSaveMode = 0,
    /// Either the location providers shouldn't be affected by battery saver, or
    /// battery saver is off.
    LocationModeNoChange = 1,
    /// The GPS based location provider should be disabled when battery saver is on
    /// and the device is non-interactive.
    LocationModeGpsDisabledWhenScreenOff = 2,
    /// All location providers should be disabled when battery saver is on and the
    /// device is non-interactive.
    LocationModeAllDisabledWhenScreenOff = 3,
    /// All the location providers will be kept available, but location fixes
    /// should only be provided to foreground apps.
    LocationModeForegroundOnly = 4,
    /// Location will not be turned off, but LocationManager will throttle all
    /// requests to providers when the device is non-interactive.
    LocationModeThrottleRequestsWhenScreenOff = 5,
}
/// Status of the battery, whether full or charging etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BatteryStatus {
    /// Battery status unknown.
    UnknownBatteryStatus = 0,
    /// Battery is being charged.
    Charging = 1,
    /// Battery is discharging.
    Discharging = 2,
    /// Battery is full.
    Full = 3,
    /// Battery is not charging.
    NotCharging = 4,
    /// Battery is low on power.
    PowerLow = 5,
}
/// Type of the charger being used to charge the battery.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PowerSource {
    /// Power source unknown.
    UnknownPowerSource = 0,
    /// Power source is an AC charger.
    Ac = 1,
    /// Power source is a USB port.
    Usb = 2,
    /// Power source is wireless.
    Wireless = 3,
    /// Battery is unplugged.
    Unplugged = 4,
}
/// A RequestHeader contains fields common to all Fleet Engine RPC requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestHeader {
    /// The BCP-47 language code, such as en-US or sr-Latn. For more information,
    /// see <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.> If none
    /// is specified, the response may be in any language, with a preference for
    /// English if such a name exists. Field value example: `en-US`.
    #[prost(string, tag="1")]
    pub language_code: ::prost::alloc::string::String,
    /// Required. CLDR region code of the region where the request originates.
    /// Field value example: `US`.
    #[prost(string, tag="2")]
    pub region_code: ::prost::alloc::string::String,
    /// Version of the calling SDK, if applicable.
    /// The version format is "major.minor.patch", example: `1.1.2`.
    #[prost(string, tag="3")]
    pub sdk_version: ::prost::alloc::string::String,
    /// Version of the operating system on which the calling SDK is running.
    /// Field value examples: `4.4.1`, `12.1`.
    #[prost(string, tag="4")]
    pub os_version: ::prost::alloc::string::String,
    /// Model of the device on which the calling SDK is running.
    /// Field value examples: `iPhone12,1`, `SM-G920F`.
    #[prost(string, tag="5")]
    pub device_model: ::prost::alloc::string::String,
    /// The type of SDK sending the request.
    #[prost(enumeration="request_header::SdkType", tag="6")]
    pub sdk_type: i32,
    /// Version of the MapSDK which the calling SDK depends on, if applicable.
    /// The version format is "major.minor.patch", example: `5.2.1`.
    #[prost(string, tag="7")]
    pub maps_sdk_version: ::prost::alloc::string::String,
    /// Version of the NavSDK which the calling SDK depends on, if applicable.
    /// The version format is "major.minor.patch", example: `2.1.0`.
    #[prost(string, tag="8")]
    pub nav_sdk_version: ::prost::alloc::string::String,
    /// Platform of the calling SDK.
    #[prost(enumeration="request_header::Platform", tag="9")]
    pub platform: i32,
    /// Manufacturer of the Android device from the calling SDK, only applicable
    /// for the Android SDKs.
    /// Field value example: `Samsung`.
    #[prost(string, tag="10")]
    pub manufacturer: ::prost::alloc::string::String,
    /// Android API level of the calling SDK, only applicable for the Android SDKs.
    /// Field value example: `23`.
    #[prost(int32, tag="11")]
    pub android_api_level: i32,
}
/// Nested message and enum types in `RequestHeader`.
pub mod request_header {
    /// Possible types of SDK.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SdkType {
        /// The default value. This value is used if the `sdk_type` is omitted.
        Unspecified = 0,
        /// The calling SDK is Consumer.
        Consumer = 1,
        /// The calling SDK is Driver.
        Driver = 2,
        /// The calling SDK is JavaScript.
        Javascript = 3,
    }
    /// The platform of the calling SDK.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Platform {
        /// The default value. This value is used if the platform is omitted.
        Unspecified = 0,
        /// The request is coming from Android.
        Android = 1,
        /// The request is coming from iOS.
        Ios = 2,
        /// The request is coming from the web.
        Web = 3,
    }
}
/// Trip metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trip {
    /// Output only. In the format "providers/{provider}/trips/{trip}"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// ID of the vehicle making this trip.
    #[prost(string, tag="2")]
    pub vehicle_id: ::prost::alloc::string::String,
    /// Current status of the trip.
    #[prost(enumeration="TripStatus", tag="3")]
    pub trip_status: i32,
    /// The type of the trip.
    #[prost(enumeration="TripType", tag="4")]
    pub trip_type: i32,
    /// Location where customer indicates they will be picked up.
    #[prost(message, optional, tag="5")]
    pub pickup_point: ::core::option::Option<TerminalLocation>,
    /// Input only. The actual location when and where customer was picked up.
    /// This field is for provider to provide feedback on actual pickup
    /// information.
    #[prost(message, optional, tag="22")]
    pub actual_pickup_point: ::core::option::Option<StopLocation>,
    /// Input only. The actual time and location of the driver arrival at
    /// the pickup point.
    /// This field is for provider to provide feedback on actual arrival
    /// information at the pickup point.
    #[prost(message, optional, tag="32")]
    pub actual_pickup_arrival_point: ::core::option::Option<StopLocation>,
    /// Output only. Either the estimated future time when the rider(s) will be picked up, or
    /// the actual time when they were picked up.
    #[prost(message, optional, tag="6")]
    pub pickup_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Intermediate stops in order that the trip requests (in addition
    /// to pickup and dropoff). Initially this will not be supported for shared
    /// trips.
    #[prost(message, repeated, tag="14")]
    pub intermediate_destinations: ::prost::alloc::vec::Vec<TerminalLocation>,
    /// Indicates the last time the `intermediate_destinations` was modified.
    /// Your server should cache this value and pass it in `UpdateTripRequest`
    /// when update `intermediate_destination_index` to ensure the
    /// Trip.intermediate_destinations is not changed.
    #[prost(message, optional, tag="25")]
    pub intermediate_destinations_version: ::core::option::Option<::prost_types::Timestamp>,
    /// When `TripStatus` is `ENROUTE_TO_INTERMEDIATE_DESTINATION`, a number
    /// between \[0..N-1\] indicating which intermediate destination the vehicle will
    /// cross next. When `TripStatus` is `ARRIVED_AT_INTERMEDIATE_DESTINATION`, a
    /// number between \[0..N-1\] indicating which intermediate destination the
    /// vehicle is at. The provider sets this value. If there are no
    /// `intermediate_destinations`, this field is ignored.
    #[prost(int32, tag="15")]
    pub intermediate_destination_index: i32,
    /// Input only. The actual time and location of the driver's arrival at
    /// an intermediate destination.
    /// This field is for provider to provide feedback on actual arrival
    /// information at intermediate destinations.
    #[prost(message, repeated, tag="33")]
    pub actual_intermediate_destination_arrival_points: ::prost::alloc::vec::Vec<StopLocation>,
    /// Input only. The actual time and location when and where the customer was picked up from
    /// an intermediate destination.
    /// This field is for provider to provide feedback on actual pickup
    /// information at intermediate destinations.
    #[prost(message, repeated, tag="34")]
    pub actual_intermediate_destinations: ::prost::alloc::vec::Vec<StopLocation>,
    /// Location where customer indicates they will be dropped off.
    #[prost(message, optional, tag="7")]
    pub dropoff_point: ::core::option::Option<TerminalLocation>,
    /// Input only. The actual time and location when and where customer was dropped off.
    /// This field is for provider to provide feedback on actual dropoff
    /// information.
    #[prost(message, optional, tag="23")]
    pub actual_dropoff_point: ::core::option::Option<StopLocation>,
    /// Output only. Either the estimated future time when the rider(s) will be dropped off at
    /// the final destination, or the actual time when they were dropped off.
    #[prost(message, optional, tag="8")]
    pub dropoff_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The full path from the current location to the dropoff point, inclusive.
    /// This path could include waypoints from other trips.
    #[prost(message, repeated, tag="16")]
    pub remaining_waypoints: ::prost::alloc::vec::Vec<TripWaypoint>,
    /// This field supports manual ordering of the waypoints for the trip. It
    /// contains all of the remaining waypoints for the assigned vehicle, as well
    /// as the pickup and drop-off waypoints for this trip. If the trip hasn't been
    /// assigned to a vehicle, then this field is ignored. For privacy reasons,
    /// this field is only populated by the server on UpdateTrip and CreateTrip
    /// calls, NOT on GetTrip calls.
    #[prost(message, repeated, tag="20")]
    pub vehicle_waypoints: ::prost::alloc::vec::Vec<TripWaypoint>,
    /// Output only. Anticipated route for this trip to the first entry in remaining_waypoints.
    /// Note that the first waypoint may belong to a different trip.
    #[prost(message, repeated, tag="9")]
    pub route: ::prost::alloc::vec::Vec<super::super::super::google::r#type::LatLng>,
    /// Output only. An encoded path to the next waypoint.
    ///
    /// Note: This field is intended only for use by the Driver SDK and Consumer
    /// SDK. Decoding is not yet supported.
    #[prost(string, tag="21")]
    pub current_route_segment: ::prost::alloc::string::String,
    /// Output only. Indicates the last time the route was modified.
    ///
    /// Note: This field is intended only for use by the Driver SDK and Consumer
    /// SDK.
    #[prost(message, optional, tag="17")]
    pub current_route_segment_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Indicates the traffic conditions along the `current_route_segment` when
    /// they're available.
    ///
    /// Note: This field is intended only for use by the Driver SDK and Consumer
    /// SDK.
    #[prost(message, optional, tag="28")]
    pub current_route_segment_traffic: ::core::option::Option<ConsumableTrafficPolyline>,
    /// Output only. Indicates the last time the `current_route_segment_traffic` was modified.
    ///
    /// Note: This field is intended only for use by the Driver SDK and Consumer
    /// SDK.
    #[prost(message, optional, tag="30")]
    pub current_route_segment_traffic_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The waypoint where `current_route_segment` ends.
    #[prost(message, optional, tag="24")]
    pub current_route_segment_end_point: ::core::option::Option<TripWaypoint>,
    /// Output only. The remaining driving distance in the `current_route_segment` field. The
    /// value is unspecified if the trip is not assigned to a vehicle, or the trip
    /// is completed or cancelled.
    #[prost(message, optional, tag="12")]
    pub remaining_distance_meters: ::core::option::Option<i32>,
    /// Output only. The ETA to the next waypoint (the first entry in the `remaining_waypoints`
    /// field). The value is unspecified if the trip is not assigned to a vehicle,
    /// or the trip is inactive (completed or cancelled).
    #[prost(message, optional, tag="13")]
    pub eta_to_first_waypoint: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The duration from when the Trip data is returned to the time in
    /// `Trip.eta_to_first_waypoint`. The value is unspecified if the trip is not
    /// assigned to a vehicle, or the trip is inactive (completed or cancelled).
    #[prost(message, optional, tag="27")]
    pub remaining_time_to_first_waypoint: ::core::option::Option<::prost_types::Duration>,
    /// Output only. Indicates the last time that `remaining_waypoints` was changed (a
    /// waypoint was added, removed, or changed).
    #[prost(message, optional, tag="19")]
    pub remaining_waypoints_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Indicates the last time the `remaining_waypoints.path_to_waypoint` and
    /// `remaining_waypoints.traffic_to_waypoint` were modified. Your client app
    /// should cache this value and pass it in `GetTripRequest` to ensure the
    /// paths and traffic for `remaining_waypoints` are only returned if updated.
    #[prost(message, optional, tag="29")]
    pub remaining_waypoints_route_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. Indicates the number of passengers on this trip and does not include the
    /// driver. A vehicle must have available capacity to be returned
    /// in SearchVehicles.
    #[prost(int32, tag="10")]
    pub number_of_passengers: i32,
    /// Output only. Indicates the last reported location of the vehicle along the route.
    #[prost(message, optional, tag="11")]
    pub last_location: ::core::option::Option<VehicleLocation>,
    /// Output only. Indicates whether the vehicle's `last_location` can be snapped to
    /// the current_route_segment. False if `last_location` or
    /// `current_route_segment` doesn't exist.
    /// It is computed by Fleet Engine. Any update from clients will be ignored.
    #[prost(bool, tag="26")]
    pub last_location_snappable: bool,
    /// The subset of Trip fields that are populated and how they should be
    /// interpreted.
    #[prost(enumeration="TripView", tag="31")]
    pub view: i32,
}
/// The actual location where a stop (pickup/dropoff) happened.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopLocation {
    /// Required. Denotes the actual location.
    #[prost(message, optional, tag="1")]
    pub point: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// Indicates when the stop happened.
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Input only. Deprecated.  Use the timestamp field.
    #[deprecated]
    #[prost(message, optional, tag="3")]
    pub stop_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The status of a trip indicating its progression.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TripStatus {
    /// Default, used for unspecified or unrecognized trip status.
    UnknownTripStatus = 0,
    /// Newly created trip.
    New = 1,
    /// The driver is on their way to the pickup point.
    EnrouteToPickup = 2,
    /// The driver has arrived at the pickup point.
    ArrivedAtPickup = 3,
    /// The driver has arrived at an intermediate destination and is waiting for
    /// the rider.
    ArrivedAtIntermediateDestination = 7,
    /// The driver is on their way to an intermediate destination
    /// (not the dropoff point).
    EnrouteToIntermediateDestination = 8,
    /// The driver has picked up the rider and is on their way to the
    /// next destination.
    EnrouteToDropoff = 4,
    /// The rider has been dropped off and the trip is complete.
    Complete = 5,
    /// The trip was canceled prior to pickup by the driver, rider, or
    /// rideshare provider.
    Canceled = 6,
}
/// A set of values that indicate upon which platform the request was issued.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BillingPlatformIdentifier {
    /// Default. Used for unspecified platforms.
    Unspecified = 0,
    /// The platform is a client server.
    Server = 1,
    /// The platform is a web browser.
    Web = 2,
    /// The platform is an Android mobile device.
    Android = 3,
    /// The platform is an IOS mobile device.
    Ios = 4,
    /// Other platforms that are not listed in this enumeration.
    Others = 5,
}
/// Selector for different sets of Trip fields in a `GetTrip` response.  See
/// \[AIP-157\](<https://google.aip.dev/157>) for context. Additional views are
/// likely to be added.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TripView {
    /// The default value. For backwards-compatibility, the API will default to an
    /// SDK view. To ensure stability and support, customers are
    /// advised to select a `TripView` other than `SDK`.
    Unspecified = 0,
    /// Includes fields that may not be interpretable or supportable using
    /// publicly available libraries.
    Sdk = 1,
    /// Trip fields are populated for the Journey Sharing use case. This view is
    /// intended for server-to-server communications.
    JourneySharingV1s = 2,
}
/// CreateTrip request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTripRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag="3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Unique Trip ID.
    /// Subject to the following restrictions:
    ///
    /// * Must be a valid Unicode string.
    /// * Limited to a maximum length of 64 characters.
    /// * Normalized according to [Unicode Normalization Form C]
    /// (<http://www.unicode.org/reports/tr15/>).
    /// * May not contain any of the following ASCII characters: '/', ':', '?',
    /// ',', or '#'.
    #[prost(string, tag="5")]
    pub trip_id: ::prost::alloc::string::String,
    /// Required. Trip entity to create.
    ///
    /// When creating a Trip, the following fields are required:
    ///
    /// * `trip_type`
    /// * `pickup_point`
    ///
    /// The following fields are used if you provide them:
    ///
    /// * `number_of_passengers`
    /// * `vehicle_id`
    /// * `dropoff_point`
    /// * `intermediate_destinations`
    ///
    /// Only `EXCLUSIVE` trips support multiple destinations.
    ///
    /// When `vehicle_id` is set for a shared trip, you must supply
    /// the list of `Trip.vehicle_waypoints` to specify the order of the remaining
    /// waypoints for the vehicle, otherwise the waypoint order will be
    /// undetermined.
    ///
    /// When you specify `Trip.vehicle_waypoints`, the list must contain all
    /// the remaining waypoints of the vehicle's trips, with no extra waypoints.
    /// You must order these waypoints such that for a given trip, the pickup
    /// point is before intermediate destinations, and all intermediate
    /// destinations come before the drop-off point. An `EXCLUSIVE` trip's
    /// waypoints must not interleave with any other trips.
    ///
    /// The `trip_id`, `waypoint_type` and `location` fields are used, and all
    /// other TripWaypoint fields in `vehicle_waypoints` are ignored.
    ///
    /// All other Trip fields are ignored.
    #[prost(message, optional, tag="4")]
    pub trip: ::core::option::Option<Trip>,
}
/// GetTrip request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTripRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format `providers/{provider}/trips/{trip}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// The subset of Trip fields that should be returned and their interpretation.
    #[prost(enumeration="TripView", tag="11")]
    pub view: i32,
    /// Indicates the minimum timestamp (exclusive) for which `Trip.route` or
    /// `Trip.current_route_segment` data are retrieved. If route data are
    /// unchanged since this timestamp, the route field is not set in the response.
    /// If a minimum is unspecified, the route data are always retrieved.
    #[prost(message, optional, tag="6")]
    pub current_route_segment_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates the minimum timestamp (exclusive) for which
    /// `Trip.remaining_waypoints` are retrieved. If they are unchanged since this
    /// timestamp, the `remaining_waypoints` are not set in the response. If this
    /// field is unspecified, `remaining_waypoints` is always retrieved.
    #[prost(message, optional, tag="7")]
    pub remaining_waypoints_version: ::core::option::Option<::prost_types::Timestamp>,
    /// The returned current route format, `LAT_LNG_LIST_TYPE` (in `Trip.route`),
    /// or `ENCODED_POLYLINE_TYPE` (in `Trip.current_route_segment`). The default
    /// is `LAT_LNG_LIST_TYPE`.
    #[prost(enumeration="PolylineFormatType", tag="8")]
    pub route_format_type: i32,
    /// Indicates the minimum timestamp (exclusive) for which
    /// `Trip.current_route_segment_traffic` is retrieved. If traffic data are
    /// unchanged since this timestamp, the `current_route_segment_traffic` field
    /// is not set in the response. If a minimum is unspecified, the traffic data
    /// are always retrieved. Note that traffic is only available for On-Demand
    /// Rides and Deliveries Solution customers.
    #[prost(message, optional, tag="9")]
    pub current_route_segment_traffic_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates the minimum timestamp (exclusive) for which
    /// `Trip.remaining_waypoints.traffic_to_waypoint` and
    /// `Trip.remaining_waypoints.path_to_waypoint` data are retrieved. If data are
    /// unchanged since this timestamp, the fields above are
    /// not set in the response. If `remaining_waypoints_route_version` is
    /// unspecified, traffic and path are always retrieved.
    #[prost(message, optional, tag="10")]
    pub remaining_waypoints_route_version: ::core::option::Option<::prost_types::Timestamp>,
}
/// ReportBillableTrip request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportBillableTripRequest {
    /// Required. Must be in the format
    /// `providers/{provider}/billableTrips/{billable_trip}`. The
    /// provider must be the Project ID (for example, `sample-cloud-project`) of
    /// the Google Cloud Project of which the service account making this call is a
    /// member.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Required. Two letter country code of the country where the trip takes place. Price is
    /// defined according to country code.
    #[prost(string, tag="3")]
    pub country_code: ::prost::alloc::string::String,
    /// The platform upon which the request was issued.
    #[prost(enumeration="BillingPlatformIdentifier", tag="5")]
    pub platform: i32,
    /// The identifiers that are directly related to the trip being reported. These
    /// are usually IDs (for example, session IDs) of pre-booking operations done
    /// before the trip ID is available. The number of `related_ids` is
    /// limited to 50.
    #[prost(string, repeated, tag="6")]
    pub related_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The type of GMP product solution (for example,
    /// `ON_DEMAND_RIDESHARING_AND_DELIVERIES`) used for the reported trip.
    #[prost(enumeration="report_billable_trip_request::SolutionType", tag="7")]
    pub solution_type: i32,
}
/// Nested message and enum types in `ReportBillableTripRequest`.
pub mod report_billable_trip_request {
    /// Selector for different solution types of a reported trip.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SolutionType {
        /// The default value. For backwards-compatibility, the API will use
        /// `ON_DEMAND_RIDESHARING_AND_DELIVERIES` by default which is the first
        /// supported solution type.
        Unspecified = 0,
        /// The solution is an on-demand ridesharing and deliveries trip.
        OnDemandRidesharingAndDeliveries = 1,
    }
}
/// UpdateTrip request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTripRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format
    /// `providers/{provider}/trips/{trip}`. The provider must
    /// be the Project ID (for example, `sample-consumer-project`) of the Google
    /// Cloud Project of which the service account making this call is a member.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The Trip associated with the update.
    ///
    /// The following fields are maintained by the Fleet Engine. Do not update
    /// them using Trip.update.
    ///
    /// * `current_route_segment`
    /// * `current_route_segment_end_point`
    /// * `current_route_segment_traffic`
    /// * `current_route_segment_traffic_version`
    /// * `current_route_segment_version`
    /// * `dropoff_time`
    /// * `eta_to_next_waypoint`
    /// * `intermediate_destinations_version`
    /// * `last_location`
    /// * `name`
    /// * `number_of_passengers`
    /// * `pickup_time`
    /// * `remaining_distance_meters`
    /// * `remaining_time_to_first_waypoint`
    /// * `remaining_waypoints`
    /// * `remaining_waypoints_version`
    /// * `route`
    ///
    /// When you update the `Trip.vehicle_id` for a shared trip, you must supply
    /// the list of `Trip.vehicle_waypoints` to specify the order of the remaining
    /// waypoints, otherwise the order will be undetermined.
    ///
    /// When you specify `Trip.vehicle_waypoints`, the list must contain all
    /// the remaining waypoints of the vehicle's trips, with no extra waypoints.
    /// You must order these waypoints such that for a given trip, the pickup
    /// point is before intermediate destinations, and all intermediate
    /// destinations come before the drop-off point. An `EXCLUSIVE` trip's
    /// waypoints must not interleave with any other trips.
    /// The `trip_id`, `waypoint_type` and `location` fields are used, and all
    /// other TripWaypoint fields in `vehicle_waypoints` are ignored.
    ///
    /// To avoid a race condition for trips with multiple destinations, you
    /// should provide `Trip.intermediate_destinations_version` when updating
    /// the trip status to `ENROUTE_TO_INTERMEDIATE_DESTINATION`. The
    /// `Trip.intermediate_destinations_version` passed must be consistent with
    /// Fleet Engine's version. If it isn't, the request fails.
    #[prost(message, optional, tag="4")]
    pub trip: ::core::option::Option<Trip>,
    /// Required. The field mask indicating which fields in Trip to update.
    /// The `update_mask` must contain at least one field.
    #[prost(message, optional, tag="5")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// SearchTrips request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTripsRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag="3")]
    pub parent: ::prost::alloc::string::String,
    /// The vehicle associated with the trips in the request. If unspecified, the
    /// returned trips do not contain:
    ///
    /// * `current_route_segment`
    /// * `remaining_waypoints`
    /// * `remaining_distance_meters`
    /// * `eta_to_first_waypoint`
    #[prost(string, tag="4")]
    pub vehicle_id: ::prost::alloc::string::String,
    /// If set to true, only Trips that influence the drivers route
    /// are included in the response.
    #[prost(bool, tag="5")]
    pub active_trips_only: bool,
    /// If not set, the server will decide the number of
    /// results to return.
    #[prost(int32, tag="6")]
    pub page_size: i32,
    /// Set this to a value previously returned in the
    /// SearchTripsResponse to continue from previous results.
    #[prost(string, tag="7")]
    pub page_token: ::prost::alloc::string::String,
    /// If specified, returns the trips that have not been updated after
    /// the time `(current - minimum_staleness)`.
    #[prost(message, optional, tag="8")]
    pub minimum_staleness: ::core::option::Option<::prost_types::Duration>,
}
/// SearchTrips response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTripsResponse {
    /// The list of trips for the requested vehicle.
    #[prost(message, repeated, tag="1")]
    pub trips: ::prost::alloc::vec::Vec<Trip>,
    /// Pass this token in the SearchTripsRequest to continue to
    /// list results. If all results have been returned, this field is an empty
    /// string or not present in the response.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod trip_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Trip management service.
    #[derive(Debug, Clone)]
    pub struct TripServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TripServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TripServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            TripServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Creates a trip in the Fleet Engine and returns the new trip.
        pub async fn create_trip(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTripRequest>,
        ) -> Result<tonic::Response<super::Trip>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.TripService/CreateTrip",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get information about a single trip.
        pub async fn get_trip(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTripRequest>,
        ) -> Result<tonic::Response<super::Trip>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.TripService/GetTrip",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Report billable trip usage.
        pub async fn report_billable_trip(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportBillableTripRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.TripService/ReportBillableTrip",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get all the trips for a specific vehicle.
        pub async fn search_trips(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchTripsRequest>,
        ) -> Result<tonic::Response<super::SearchTripsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.TripService/SearchTrips",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates trip data.
        pub async fn update_trip(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTripRequest>,
        ) -> Result<tonic::Response<super::Trip>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.TripService/UpdateTrip",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// `CreateVehicle` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVehicleRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag="3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Unique Vehicle ID.
    /// Subject to the following restrictions:
    ///
    /// * Must be a valid Unicode string.
    /// * Limited to a maximum length of 64 characters.
    /// * Normalized according to [Unicode Normalization Form C]
    /// (<http://www.unicode.org/reports/tr15/>).
    /// * May not contain any of the following ASCII characters: '/', ':', '?',
    /// ',', or '#'.
    #[prost(string, tag="4")]
    pub vehicle_id: ::prost::alloc::string::String,
    /// Required. The Vehicle entity to create. When creating a Vehicle, the following
    /// fields are required:
    ///
    /// * `vehicleState`
    /// * `supportedTripTypes`
    /// * `maximumCapacity`
    /// * `vehicleType`
    ///
    /// When creating a Vehicle, the following fields are ignored:
    ///
    /// * `name`
    /// * `currentTrips`
    /// * `availableCapacity`
    /// * `current_route_segment`
    /// * `current_route_segment_end_point`
    /// * `current_route_segment_version`
    /// * `current_route_segment_traffic`
    /// * `route`
    /// * `waypoints`
    /// * `waypoints_version`
    /// * `remaining_distance_meters`
    /// * `remaining_time_seconds`
    /// * `eta_to_next_waypoint`
    /// * `navigation_status`
    ///
    /// All other fields are optional and used if provided.
    #[prost(message, optional, tag="5")]
    pub vehicle: ::core::option::Option<Vehicle>,
}
/// `GetVehicle` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVehicleRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format
    /// `providers/{provider}/vehicles/{vehicle}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Indicates the minimum timestamp (exclusive) for which
    /// `Vehicle.current_route_segment` is retrieved.
    /// If the route is unchanged since this timestamp, the `current_route_segment`
    /// field is not set in the response. If a minimum is unspecified, the
    /// `current_route_segment` is always retrieved.
    #[prost(message, optional, tag="4")]
    pub current_route_segment_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates the minimum timestamp (exclusive) for which `Vehicle.waypoints`
    /// data is retrieved. If the waypoints are unchanged since this timestamp, the
    /// `vehicle.waypoints` data is not set in the response. If this field is
    /// unspecified, `vehicle.waypoints` is always retrieved.
    #[prost(message, optional, tag="5")]
    pub waypoints_version: ::core::option::Option<::prost_types::Timestamp>,
}
/// `UpdateVehicle request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVehicleRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format
    /// `providers/{provider}/vehicles/{vehicle}`.
    /// The {provider} must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The `Vehicle` entity values to apply.  When updating a `Vehicle`,
    /// the following fields may not be updated as they are managed by the
    /// server.
    ///
    /// * `current_trips`
    /// * `available_capacity`
    /// * `current_route_segment_version`
    /// * `waypoints_version`
    ///
    /// Furthermore, the vehicle `name` cannot be updated.
    ///
    /// If the `attributes` field is updated, **all** the vehicle's attributes are
    /// replaced with the attributes provided in the request. If you want to update
    /// only some attributes, see the `UpdateVehicleAttributes` method. Likewise,
    /// the `waypoints` field can be updated, but must contain all the waypoints.
    /// currently on the vehicle, and no other waypoints.
    #[prost(message, optional, tag="4")]
    pub vehicle: ::core::option::Option<Vehicle>,
    /// Required. A field mask indicating which fields of the `Vehicle` to update.
    /// At least one field name must be provided.
    #[prost(message, optional, tag="5")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// `UpdateVehicleLocation` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVehicleLocationRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format
    /// `providers/{provider}/vehicles/{vehicle}`.
    /// The {provider} must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The vehicle's most recent location.  The `location` and `update_time`
    /// subfields are required.
    #[prost(message, optional, tag="4")]
    pub current_location: ::core::option::Option<VehicleLocation>,
    /// Set the vehicle's state to either `ONLINE` or `OFFLINE`.
    /// If set to `UNKNOWN_VEHICLE_STATE`, the vehicle's state will not be altered.
    #[prost(enumeration="VehicleState", tag="5")]
    pub current_state: i32,
}
/// `UpdateVehicleAttributes` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVehicleAttributesRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format `providers/{provider}/vehicles/{vehicle}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The vehicle attributes to update. Unmentioned attributes will not be
    /// altered or removed.
    #[prost(message, repeated, tag="4")]
    pub attributes: ::prost::alloc::vec::Vec<VehicleAttribute>,
}
/// `UpdateVehicleAttributes` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVehicleAttributesResponse {
    /// Required. The updated full list of vehicle attributes, including new,
    /// altered and untouched attributes.
    #[prost(message, repeated, tag="1")]
    pub attributes: ::prost::alloc::vec::Vec<VehicleAttribute>,
}
/// `SearchVehicles` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchVehiclesRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag="3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The pickup point to search near.
    #[prost(message, optional, tag="4")]
    pub pickup_point: ::core::option::Option<TerminalLocation>,
    /// The customer's intended dropoff location. The field is required if
    /// `trip_types` contains `TripType.SHARED`.
    #[prost(message, optional, tag="5")]
    pub dropoff_point: ::core::option::Option<TerminalLocation>,
    /// Required. Defines the vehicle search radius around the pickup point. Only
    /// vehicles within the search radius will be returned. Value must be between
    /// 400 and 10000 meters (inclusive).
    #[prost(int32, tag="6")]
    pub pickup_radius_meters: i32,
    /// Required. Specifies the maximum number of vehicles to return. The value
    /// must be between 1 and 50 (inclusive).
    #[prost(int32, tag="7")]
    pub count: i32,
    /// Required. Specifies the number of passengers being considered for a trip. The
    /// value must be greater than or equal to one. The driver is not considered in
    /// the capacity value.
    #[prost(int32, tag="8")]
    pub minimum_capacity: i32,
    /// Required. Represents the type of proposed trip. Eligible vehicles are those
    /// that can support at least one of the specified trip type.
    ///
    /// `EXCLUSIVE` and `SHARED` may not be included together.
    /// `SHARED` is not supported when `current_trips_present` is
    /// `CURRENT_TRIPS_PRESENT_UNSPECIFIED`.
    #[prost(enumeration="TripType", repeated, packed="false", tag="9")]
    pub trip_types: ::prost::alloc::vec::Vec<i32>,
    /// Restricts the search to only those vehicles that have updated their
    /// locations within the specified duration. If this field is not
    /// set, the server uses five minutes as the default value.
    #[prost(message, optional, tag="10")]
    pub maximum_staleness: ::core::option::Option<::prost_types::Duration>,
    /// Required. Restricts the search to vehicles with one of the specified types.
    /// At least one vehicle type must be specified.
    #[prost(message, repeated, tag="14")]
    pub vehicle_types: ::prost::alloc::vec::Vec<vehicle::VehicleType>,
    /// Callers can form complex logical operations using any combination of the
    /// `required_attributes`, `required_one_of_attributes`, and
    /// `required_one_of_attribute_sets` fields.
    ///
    /// `required_attributes` is a list; `required_one_of_attributes` uses a
    /// message which allows a list of lists. In combination, the two fields allow
    /// the composition of this expression:
    ///
    /// ```
    /// (required_attributes\[0\] AND required_attributes\[1\] AND ...)
    /// AND
    /// (required_one_of_attributes\[0][0\] OR required_one_of_attributes\[0][1\] OR
    /// ...)
    /// AND
    /// (required_one_of_attributes\[1][0\] OR required_one_of_attributes\[1][1\] OR
    /// ...)
    /// ```
    ///
    /// Restricts the search to only those vehicles with the specified attributes.
    /// This field is a conjunction/AND operation. Your app can specify up to 100
    /// attributes; however, the combined key:value string length cannot exceed
    /// 1024 characters.
    #[prost(message, repeated, tag="12")]
    pub required_attributes: ::prost::alloc::vec::Vec<VehicleAttribute>,
    /// Restricts the search to only those vehicles with at least one of
    /// the specified attributes in each `VehicleAttributeList`. Within each
    /// list, a vehicle must match at least one of the attributes. This field is an
    /// inclusive disjunction/OR operation in each `VehicleAttributeList` and a
    /// conjunction/AND operation across the collection of `VehicleAttributeList`.
    #[prost(message, repeated, tag="15")]
    pub required_one_of_attributes: ::prost::alloc::vec::Vec<VehicleAttributeList>,
    /// `required_one_of_attribute_sets` provides additional functionality.
    ///
    /// Similar to `required_one_of_attributes`, `required_one_of_attribute_sets`
    /// uses a message which allows a list of lists, allowing expressions such as
    /// this one:
    ///
    /// ```
    /// (required_attributes\[0\] AND required_attributes\[1\] AND ...)
    /// AND
    /// (required_one_of_attribute_sets\[0][0\] AND
    /// required_one_of_attribute_sets\[0][1\] AND
    /// ...)
    /// OR
    /// (required_one_of_attribute_sets\[1][0\] AND
    /// required_one_of_attribute_sets\[1][1\] AND
    /// ...)
    /// ```
    ///
    /// Restricts the search to only those vehicles with all the attributes in a
    /// `VehicleAttributeList`. Within each list, a
    /// vehicle must match all of the attributes. This field is a conjunction/AND
    /// operation in each `VehicleAttributeList` and inclusive disjunction/OR
    /// operation across the collection of `VehicleAttributeList`.
    #[prost(message, repeated, tag="20")]
    pub required_one_of_attribute_sets: ::prost::alloc::vec::Vec<VehicleAttributeList>,
    /// Required. Specifies the desired ordering criterion for results.
    #[prost(enumeration="search_vehicles_request::VehicleMatchOrder", tag="13")]
    pub order_by: i32,
    /// Indicates if a vehicle with a single active trip is eligible for another
    /// match. If `false`, vehicles with assigned trips are excluded from the
    /// search results. If `true`, search results include vehicles with
    /// `TripStatus` of `ENROUTE_TO_DROPOFF`.
    ///
    /// This field is only considered if a single `trip_type` of `EXCLUSIVE` is
    /// specified.
    ///
    /// The default value is `false`.
    #[prost(bool, tag="18")]
    pub include_back_to_back: bool,
    /// Indicates the trip associated with this `SearchVehicleRequest`.
    #[prost(string, tag="19")]
    pub trip_id: ::prost::alloc::string::String,
    /// Restricts vehicles from appearing in the search results based on
    /// their current trips.
    ///
    /// When current_trips_present is `NONE` or `ANY`, `trip_types` can be either
    /// `EXCLUSIVE` or `SHARED`, but not both.
    #[prost(enumeration="search_vehicles_request::CurrentTripsPresent", tag="21")]
    pub current_trips_present: i32,
}
/// Nested message and enum types in `SearchVehiclesRequest`.
pub mod search_vehicles_request {
    /// Specifies the order of the vehicle matches in the response.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VehicleMatchOrder {
        /// Default, used for unspecified or unrecognized vehicle matches order.
        UnknownVehicleMatchOrder = 0,
        /// Ascending order by vehicle driving time to the pickup point.
        PickupPointEta = 1,
        /// Ascending order by vehicle driving distance to the pickup point.
        PickupPointDistance = 2,
        /// Ascending order by vehicle driving time to the dropoff point. This order
        /// can only be used if the dropoff point is specified in the request.
        DropoffPointEta = 3,
        /// Ascending order by straight-line distance from the vehicle's last
        /// reported location to the pickup point.
        PickupPointStraightDistance = 4,
        /// Ascending order by the configured match cost.
        Cost = 5,
    }
    /// Specifies the types of restrictions on a vehicle's current trips.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CurrentTripsPresent {
        /// Only vehicles without trips can appear in search results.
        /// A validation exception is thrown if `include_back_to_back` is true. See
        /// the `include_back_to_back` flag for more details.
        Unspecified = 0,
        /// Vehicles without trips can appear in search results.
        /// A validation exception is thrown if `include_back_to_back` is true.
        None = 1,
        /// Vehicles with at most 5 current trips and 10 waypoints are included
        /// in the search results.
        /// A validation exception is thrown if `include_back_to_back` is true.
        Any = 2,
    }
}
/// `SearchVehicles` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchVehiclesResponse {
    /// List of vehicles that match the `SearchVehiclesRequest` criteria, ordered
    /// according to `SearchVehiclesRequest.order_by` field.
    #[prost(message, repeated, tag="1")]
    pub matches: ::prost::alloc::vec::Vec<VehicleMatch>,
}
/// `ListVehicles` request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVehiclesRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag="12")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of vehicles to return.
    /// Default value: 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// The `next_page_token` value returned from a previous call to
    /// `ListVehicles`. Functionality is undefined if the filter criteria of this
    /// request don't match the criteria in the request that produced this
    /// `page_token`.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
    /// Specifies the required minimum capacity of the vehicle. All vehicles
    /// returned will have a `maximum_capacity` greater than or equal to this
    /// value. If set, must be greater or equal to 0.
    #[prost(message, optional, tag="6")]
    pub minimum_capacity: ::core::option::Option<i32>,
    /// Restrict the response to vehicles that support at least
    /// one of the specified trip types.
    #[prost(enumeration="TripType", repeated, tag="7")]
    pub trip_types: ::prost::alloc::vec::Vec<i32>,
    /// Restrict the response to vehicles that have updated
    /// their locations within the specified duration back from now.
    /// If present, must be a valid positive duration.
    #[prost(message, optional, tag="8")]
    pub maximum_staleness: ::core::option::Option<::prost_types::Duration>,
    /// Required. Restrict the response to vehicles with one of the specified type
    /// categories.
    #[prost(enumeration="vehicle::vehicle_type::Category", repeated, packed="false", tag="9")]
    pub vehicle_type_categories: ::prost::alloc::vec::Vec<i32>,
    /// Callers can form complex logical operations using any combination of the
    /// `required_attributes`, `required_one_of_attributes`, and
    /// `required_one_of_attribute_sets` fields.
    ///
    /// `required_attributes` is a list; `required_one_of_attributes` uses a
    /// message which allows a list of lists. In combination, the two fields allow
    /// the composition of this expression:
    ///
    /// ```
    /// (required_attributes\[0\] AND required_attributes\[1\] AND ...)
    /// AND
    /// (required_one_of_attributes\[0][0\] OR required_one_of_attributes\[0][1\] OR
    /// ...)
    /// AND
    /// (required_one_of_attributes\[1][0\] OR required_one_of_attributes\[1][1\] OR
    /// ...)
    /// ```
    ///
    /// Restrict the response to vehicles with the specified attributes. This field
    /// is a conjunction/AND operation. Your app can specify up to 100 attributes;
    /// however, the combined key:value string length cannot exceed 1024
    /// characters.
    #[prost(string, repeated, tag="10")]
    pub required_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Restrict the response to vehicles with at least one
    /// of the specified attributes in each `VehicleAttributeList`.
    /// Within each list, a vehicle must match at least one of the attributes.
    /// This field is an inclusive disjunction/OR operation in each
    /// `VehicleAttributeList` and a conjunction/AND operation across the
    /// collection of `VehicleAttributeList`. Format:
    /// key1:value1|key2:value2|key3:value3...
    #[prost(string, repeated, tag="13")]
    pub required_one_of_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// `required_one_of_attribute_sets` provides additional functionality.
    ///
    /// Similar to `required_one_of_attributes`, `required_one_of_attribute_sets`
    /// uses a message which allows a list of lists, allowing expressions such as
    /// this one:
    ///
    /// ```
    /// (required_attributes\[0\] AND required_attributes\[1\] AND ...)
    /// AND
    /// (required_one_of_attributes\[0][0\] AND required_one_of_attributes\[0][1\] AND
    /// ...)
    /// OR
    /// (required_one_of_attributes\[1][0\] AND required_one_of_attributes\[1][1\] AND
    /// ...)
    /// ```
    ///
    /// Restrict the response to vehicles that match all the attributes in a
    /// `VehicleAttributeList`. Within each list, a vehicle must match all of the
    /// attributes. This field is a conjunction/AND operation in each
    /// `VehicleAttributeList` and inclusive disjunction/OR operation across the
    /// collection of `VehicleAttributeList`. Format:
    /// key1:value1|key2:value2|key3:value3...
    #[prost(string, repeated, tag="15")]
    pub required_one_of_attribute_sets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Restrict the response to vehicles that have this vehicle state.
    #[prost(enumeration="VehicleState", tag="11")]
    pub vehicle_state: i32,
    /// Only return the vehicles with current trip(s).
    #[prost(bool, tag="14")]
    pub on_trip_only: bool,
}
/// `ListVehicles` response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVehiclesResponse {
    /// Vehicles matching the criteria in the request.
    /// The maximum number of vehicles returned is determined by the `page_size`
    /// field in the request.
    #[prost(message, repeated, tag="1")]
    pub vehicles: ::prost::alloc::vec::Vec<Vehicle>,
    /// Token to retrieve the next page of vehicles, or empty if there are no
    /// more vehicles that meet the request criteria.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Required. Total number of vehicles matching the request criteria across all pages.
    #[prost(int64, tag="3")]
    pub total_size: i64,
}
/// Describes intermediate points along a route.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Waypoint {
    /// The location of this waypoint.
    #[prost(message, optional, tag="1")]
    pub lat_lng: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// The estimated time that the vehicle will arrive at this waypoint.
    #[prost(message, optional, tag="2")]
    pub eta: ::core::option::Option<::prost_types::Timestamp>,
}
/// Contains the vehicle and related estimates for a vehicle that match the
/// points of active trips for the vehicle `SearchVehiclesRequest`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleMatch {
    /// Required. A vehicle that matches the request.
    #[prost(message, optional, tag="1")]
    pub vehicle: ::core::option::Option<Vehicle>,
    /// The vehicle's driving ETA to the pickup point specified in the
    /// request. An empty value indicates a failure in calculating ETA for the
    /// vehicle.  If `SearchVehiclesRequest.include_back_to_back` was `true` and
    /// this vehicle has an active trip, `vehicle_pickup_eta` includes the time
    /// required to complete the current active trip.
    #[prost(message, optional, tag="2")]
    pub vehicle_pickup_eta: ::core::option::Option<::prost_types::Timestamp>,
    /// The distance from the Vehicle's current location to the pickup point
    /// specified in the request, including any intermediate pickup or dropoff
    /// points for existing trips. This distance comprises the calculated driving
    /// (route) distance, plus the straight line distance between the navigation
    /// end point and the requested pickup point. (The distance between the
    /// navigation end point and the requested pickup point is typically small.) An
    /// empty value indicates an error in calculating the distance.
    #[prost(message, optional, tag="3")]
    pub vehicle_pickup_distance_meters: ::core::option::Option<i32>,
    /// Required. The straight-line distance between the vehicle and the pickup point
    /// specified in the request.
    #[prost(message, optional, tag="11")]
    pub vehicle_pickup_straight_line_distance_meters: ::core::option::Option<i32>,
    /// The complete vehicle's driving ETA to the drop off point specified in the
    /// request. The ETA includes stopping at any waypoints before the
    /// `dropoff_point` specified in the request. The value will only be populated
    /// when a drop off point is specified in the request. An empty value indicates
    /// an error calculating the ETA.
    #[prost(message, optional, tag="4")]
    pub vehicle_dropoff_eta: ::core::option::Option<::prost_types::Timestamp>,
    /// The vehicle's driving distance (in meters) from the pickup point
    /// to the drop off point specified in the request. The distance is only
    /// between the two points and does not include the vehicle location or any
    /// other points that must be visited before the vehicle visits either the
    /// pickup point or dropoff point. The value will only be populated when a
    /// `dropoff_point` is specified in the request. An empty value indicates
    /// a failure in calculating the distance from the pickup to
    /// drop off point specified in the request.
    #[prost(message, optional, tag="5")]
    pub vehicle_pickup_to_dropoff_distance_meters: ::core::option::Option<i32>,
    /// Required. The trip type of the request that was used to calculate the ETA
    /// to the pickup point.
    #[prost(enumeration="TripType", tag="6")]
    pub trip_type: i32,
    /// The ordered list of waypoints used to calculate the ETA. The list
    /// includes vehicle location, the pickup/drop off points of active
    /// trips for the vehicle, and the pickup/drop off points provided in the
    /// request. An empty list indicates a failure in calculating ETA for the
    /// vehicle.
    #[prost(message, repeated, tag="7")]
    pub vehicle_trips_waypoints: ::prost::alloc::vec::Vec<Waypoint>,
    /// Type of the vehicle match.
    #[prost(enumeration="vehicle_match::VehicleMatchType", tag="8")]
    pub vehicle_match_type: i32,
    /// The order requested for sorting vehicle matches.
    #[prost(enumeration="search_vehicles_request::VehicleMatchOrder", tag="9")]
    pub requested_ordered_by: i32,
    /// The actual order that was used for this vehicle. Normally this
    /// will match the 'order_by' field from the request; however, in certain
    /// circumstances such as an internal server error, a different method
    /// may be used (such as `PICKUP_POINT_STRAIGHT_DISTANCE`).
    #[prost(enumeration="search_vehicles_request::VehicleMatchOrder", tag="10")]
    pub ordered_by: i32,
}
/// Nested message and enum types in `VehicleMatch`.
pub mod vehicle_match {
    /// Type of vehicle match.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VehicleMatchType {
        /// Unknown vehicle match type
        Unknown = 0,
        /// The vehicle currently has no trip assigned to it and can proceed to the
        /// pickup point.
        Exclusive = 1,
        /// The vehicle is currently assigned to a trip, but can proceed to the
        /// pickup point after completing the in-progress trip.  ETA and distance
        /// calculations take the existing trip into account.
        BackToBack = 2,
        /// The vehicle has sufficient capacity for a shared ride.
        Carpool = 3,
        /// The vehicle will finish its current, active trip before proceeding to the
        /// pickup point.  ETA and distance calculations take the existing trip into
        /// account.
        CarpoolBackToBack = 4,
    }
}
/// A list-of-lists datatype for vehicle attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleAttributeList {
    /// A list of attributes in this collection.
    #[prost(message, repeated, tag="1")]
    pub attributes: ::prost::alloc::vec::Vec<VehicleAttribute>,
}
/// Generated client implementations.
pub mod vehicle_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Vehicle management service.
    #[derive(Debug, Clone)]
    pub struct VehicleServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VehicleServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VehicleServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            VehicleServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Instantiates a new vehicle associated with an on-demand rideshare or
        /// deliveries provider. Each `Vehicle` must have a unique vehicle ID.
        ///
        /// The following `Vehicle` fields are required when creating a `Vehicle`:
        ///
        /// * `vehicleState`
        /// * `supportedTripTypes`
        /// * `maximumCapacity`
        /// * `vehicleType`
        ///
        /// The following `Vehicle` fields are ignored when creating a `Vehicle`:
        ///
        /// * `name`
        /// * `currentTrips`
        /// * `availableCapacity`
        /// * `current_route_segment`
        /// * `current_route_segment_end_point`
        /// * `current_route_segment_version`
        /// * `current_route_segment_traffic`
        /// * `route`
        /// * `waypoints`
        /// * `waypoints_version`
        /// * `remaining_distance_meters`
        /// * `remaining_time_seconds`
        /// * `eta_to_next_waypoint`
        /// * `navigation_status`
        ///
        /// All other fields are optional and used if provided.
        pub async fn create_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVehicleRequest>,
        ) -> Result<tonic::Response<super::Vehicle>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/CreateVehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a vehicle from the Fleet Engine.
        pub async fn get_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVehicleRequest>,
        ) -> Result<tonic::Response<super::Vehicle>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/GetVehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Writes updated vehicle data to the Fleet Engine.
        ///
        /// When updating a `Vehicle`, the following fields cannot be updated since
        /// they are managed by the server:
        ///
        /// * `currentTrips`
        /// * `availableCapacity`
        /// * `current_route_segment_version`
        /// * `waypoints_version`
        ///
        /// The vehicle `name` also cannot be updated.
        ///
        /// If the `attributes` field is updated, **all** the vehicle's attributes are
        /// replaced with the attributes provided in the request. If you want to update
        /// only some attributes, see the `UpdateVehicleAttributes` method. Likewise,
        /// the `waypoints` field can be updated, but must contain all the waypoints
        /// currently on the vehicle, and no other waypoints.
        pub async fn update_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVehicleRequest>,
        ) -> Result<tonic::Response<super::Vehicle>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/UpdateVehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deprecated: Use the `UpdateVehicle` method instead.
        /// UpdateVehicleLocation updates the location of the vehicle.
        pub async fn update_vehicle_location(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVehicleLocationRequest>,
        ) -> Result<tonic::Response<super::VehicleLocation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/UpdateVehicleLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Partially updates a vehicle's attributes.
        /// Only the attributes mentioned in the request will be updated, other
        /// attributes will NOT be altered. Note: this is different in `UpdateVehicle`,
        /// where the whole `attributes` field will be replaced by the one in
        /// `UpdateVehicleRequest`, attributes not in the request would be removed.
        pub async fn update_vehicle_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVehicleAttributesRequest>,
        ) -> Result<
            tonic::Response<super::UpdateVehicleAttributesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/UpdateVehicleAttributes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a paginated list of vehicles associated with
        /// a provider that match the request options.
        pub async fn list_vehicles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVehiclesRequest>,
        ) -> Result<tonic::Response<super::ListVehiclesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/ListVehicles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a list of vehicles that match the request options.
        pub async fn search_vehicles(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchVehiclesRequest>,
        ) -> Result<tonic::Response<super::SearchVehiclesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/SearchVehicles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a list of vehicles that match the request
        /// options, but the vehicle locations will be somewhat altered for privacy.
        /// This method does not support the `SearchVehicleRequest.order_by` field.
        /// Vehicle matches in the response will be in order of distance from the
        /// pickup point.  Only the `vehicle` and `trip_type` fields will be populated.
        pub async fn search_fuzzed_vehicles(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchVehiclesRequest>,
        ) -> Result<tonic::Response<super::SearchVehiclesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/SearchFuzzedVehicles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
