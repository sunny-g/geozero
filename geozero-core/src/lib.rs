//! Collection of GeoZero API implementations.

mod geojson_reader;
mod geojson_writer;
#[cfg(feature = "geos")]
mod geos_reader;
#[cfg(feature = "geos")]
mod geos_writer;
mod rustgeo_writer;
/// SVG Writer.
pub mod svg;
mod wkt_writer;

/// GeoJSON Reader + Writer.
pub mod geojson {
    pub use crate::geojson_reader::*;
    pub use crate::geojson_writer::*;
}

/// [georust/geo](https://github.com/georust/geo) Writer.
pub mod geo {
    pub use crate::rustgeo_writer::*;
}

/// WKT Writer.
pub mod wkt {
    pub use crate::wkt_writer::*;
}

/// [GEOS](https://github.com/georust/geos) Reader + Writer.
#[cfg(feature = "geos")]
pub mod geos {
    pub use crate::geos_reader::*;
    pub use crate::geos_writer::*;
}
