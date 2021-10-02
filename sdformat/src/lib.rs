use serde::{Deserialize, Serialize};

/// SDFormat base element that can include one model, actor, light, or worlds.
/// A user of multiple worlds could run parallel instances of simulation,
/// or offer selection of a world at runtime.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename="sdf", rename_all = "snake_case")]
pub struct Sdf {
    #[serde(default = "Sdf::default_version")]
    /// Version number of the SDFormat specification.
    version: String,
    #[serde(rename = "$value")]
    sdf_type: SdfType,
    // world: World,
    // model: Model,
    // actor: Actor,
    // light: Light,
}

impl Sdf {
    const DEFAULT_VERSION: &'static str = "1.8";

    pub fn default_version() -> String {
        String::from(Sdf::DEFAULT_VERSION)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SdfType {
    World(World),
    Model(Model),
    Actor,
    Light,
}

/// The world element encapsulates an entire world description including: models, scene, physics, and plugins.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]

pub struct World {}

fn true_default() -> bool {
    true
}

fn some_true_default() -> Option<bool> {
    Some(true)
}

fn some_false_default() -> Option<bool> {
    Some(false)
}

/// A position(x,y,z) and orientation(roll, pitch yaw) with respect to the frame named in the relative_to attribute.
/// Default: 0 0 0 0 -0 0
/// TODO: Create `Pose`
pub type Pose = String;

/// The model element defines a complete robot or any other physical object.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Model {
    /// A unique name for the model. This name must not match another model in the world.
    pub name: String,
    /// The name of the model's canonical link, to which the model's implicit coordinate frame is attached. If unset or set to an empty string, the first link element listed as a child of this model is chosen as the canonical link.
    #[serde(default)]
    pub canonical_link: Option<String>,
    /// The frame inside this model whose pose will be set by the pose element of the model. i.e, the pose element specifies the pose of this frame instead of the model frame.
    #[serde(default)]
    pub placement_frame: Option<String>,
    /// If set to true, the model is immovable. Otherwise the model is simulated in the dynamics engine.
    #[serde(default)]
    pub r#static: Option<bool>,
    /// If set to true, all links in the model will collide with each other (except those connected by a joint). Can be overridden by the link or collision element self_collide property. Two links within a model will collide if link1.self_collide OR link2.self_collide. Links connected by a joint will never collide.
    #[serde(default)]
    pub self_collide: Option<bool>,
    /// Allows a model to auto-disable, which is means the physics engine can skip updating the model when the model is at rest. This parameter is only used by models with no joints.
    #[serde(default = "some_true_default")]
    pub allow_auto_disable: Option<bool>,
    #[serde(default)]
    pub include: Vec<Include>,
    #[serde(default)]
    pub model: Vec<Model>,
    /// If set to true, all links in the model will be affected by the wind. Can be overriden by the link wind property.
    #[serde(default = "some_false_default")]
    pub enable_wind: Option<bool>,
    #[serde(default)]
    pub frame: Vec<Frame>,
    #[serde(default)]
    pub pose: Option<Pose>,
    #[serde(default)]
    pub link: Vec<Link>,
    #[serde(default)]
    pub joint: Vec<Joint>,
    #[serde(default)]
    pub plugin: Vec<Plugin>,
    #[serde(default)]
    pub gripper: Vec<Gripper>,
}

/// Include resources from a URI. This can be used to nest models. Included resources can only contain one 'model', 'light' or 'actor' element. The URI can point to a directory or a file. If the URI is a directory, it must conform to the model database structure (see /tutorials?tut=composition&cat=specification&#defining-models-in-separate-files).
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Include {
    /// URI to a resource, such as a model
    pub uri: String,
    /// Override the name of the included model.
    pub name: Option<String>,
    /// Override the static value of the included model.
    pub r#static: Option<bool>,
    /// The frame inside the included model whose pose will be set by the specified pose element. If this element is specified, the pose must be specified.
    pub placement_frame: Option<String>,
    pub pose: Option<Pose>,
    pub plugin: Vec<Plugin>,
}

/// A plugin is a dynamically loaded chunk of code. It can exist as a child of world, model, and sensor.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Plugin {
    /// A unique name for the plugin, scoped to its parent.
    name: String,
    /// Name of the shared library to load. If the filename is not a full path name,
    /// the file will be searched for in the configuration paths.
    filename: String,
}

/// A frame of reference to which a pose is relative.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Frame {
    /// Name of the frame. This name must not match another frame defined inside the parent that this frame is attached to.
    name: String,
    /// Name of the link or frame to which this frame is attached. If a frame is specified, recursively following the attached_to attributes of the specified frames must lead to the name of a link, a model, or the world frame.
    pub attached_to: Option<String>,
    pub pose: Option<Pose>,
}

/// A nested model element
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NestedModel {
    /// A unique name for the model. This name must not match another nested model in the same level as this model.
    name: String,
}


/// A physical link with inertia, collision, and visual properties.
/// A link must be a child of a model, and any number of links may exist in a model.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Link {
    name: String,
    // TODO: rest of elements
}

/// /// A joint connects two links with kinematic and dynamic properties. By default, the pose of a joint is expressed in the child link frame.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Joint {
    // TODO: Rest of attributes and elements
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Gripper {
    pub name: String,
    pub grasp_check: Option<GraspCheck>,
    pub gripper_link: Option<String>,
    pub palm_link: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraspCheck {
    /// default: 40
    // TODO: add default
    pub detach_step: Option<i64>,
    /// default: 20
    // TODO: add default
    pub attach_step: Option<i64>,
    /// default: 2
    // TODO: add default
    pub min_contact_count: Option<u64>,
}

/// A special kind of model which can have a scripted motion.
/// This includes both global waypoint type animations and skeleton animations.
#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {}
/// The light element describes a light sou
#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    // A unique name for the light
    name: String,
    // The light type: point, directional spot
    #[serde(default)]
    light_type: String,
    // When true, the light will cast shadows
    #[serde(default)]
    cast_shadows: Option<bool>,
    // Scale factor to set the relative power of a light
    #[serde(default)]
    intensity: Option<f64>,
    // Diffuse light color
    #[serde(default)]
    diffuse: Option<Color>,
    // Specular light color
    #[serde(default)]
    specular: Option<Color>,
    // Light attenuation
    #[serde(default)]
    attenuation: Vec<Attenuation>,
    // Direction of light, only applicable for spot and directional light
    #[serde(default)]
    direction: Vec<i32>,
    // Spot light parameters
    #[serde(default)]
    spot: Vec<Spot>,
    // A position and orientation with respect to the frame named in relative_to attribute
    #[serde(default)]
    pose: Vec<Pose>,
}

// Light attenuation
pub struct Attenuation {
    range: f64,
    linear: Option<f64>,
    constant: Option<f64>,
    quadratic: Option<f64>,
}

// Spot light parameters
pub struct Spot {
    inner_angle: f64,
    outer_angle: f64,
    falloff: f64,
}

// A position and orientation with respect to the frame named in relative_to attribute
pub struct Pose {
    relative_to: Option<String>,
}

// Unit struct for tpe `color`
struct Color(u8, u8, u8);

#[cfg(test)]
mod test {
    use super::*;
    use serde_xml_rs::{from_reader, from_str, to_string};
    use pretty_assertions::assert_eq;

    /// **NOTE:** Version 1.5
    pub static PX4_TYPHOON_WORLD: &'static str = include_str!("../tests/px4_typhoon_h480.world");

    #[test]
    fn test_px4_typhoon_world() {
        let sdf: Sdf = from_str(PX4_TYPHOON_WORLD).unwrap();

        let xml_string = to_string(&sdf).expect("Should serialize to string");

        assert_eq!(PX4_TYPHOON_WORLD, xml_string)
    }

    #[test]
    fn test_sdf() {
        let sdf_xml = r#"
            <?xml version="1.0" ?>
                <sdf version="1.8">
                    <model name="box">
                        <pose>0 0 0.5 0 0 0</pose>
                        <static>false</static>
                        <self_collide>true</self_collide>
                    </model>
                </sdf>"#;

        //     let s = r##"
        //     <Project name="my_project">
        //         <Item name="hello" source="world.rs" />
        //     </Project>
        // "##;
        let sdf: Sdf = from_reader(sdf_xml.as_bytes()).unwrap();

        let expected = Sdf {
            version: String::from("1.8"),
            sdf_type: SdfType::Model(Model {
                name: "box".into(),
                canonical_link: None,
                placement_frame: None,
                r#static: Some(false),
                self_collide: Some(true),
                pose: Some("0 0 0.5 0 0 0".into()),
                allow_auto_disable: Some(true),
                include: vec![],
                model: vec![],
                enable_wind: Some(false),
                frame: vec![],
                link: vec![],
                joint: vec![],
                plugin: vec![],
                gripper: vec![],
            }),
        };
        assert_eq!(expected, sdf);

        let model = r#"<model name="box">
                    <pose>0 0 0.5 0 0 0</pose>
                    <static>false</static>
                    <self_collide>true</self_collide>

                    <link name="link">
                    ...
                    </link>

                    <joint type="revolute" name="my_joint">
                    ...
                    </joint>

                    <plugin filename="libMyPlugin.so" name="my_plugin"/>

                </model>"#;
    }

    #[test]
    #[ignore]
    fn test_model() {
        let model_xml = r#"
            <?xml version="1.0" ?>
                <sdf version="1.5">
                <model name="box">
                    <pose>0 0 0.5 0 0 0</pose>
                    <static>false</static>

                    <link name="link">
                    ...
                    </link>

                    <joint type="revolute" name="my_joint">
                    ...
                    </joint>

                    <plugin filename="libMyPlugin.so" name="my_plugin"/>

                </model>
                </sdf>"#;
    }
}
