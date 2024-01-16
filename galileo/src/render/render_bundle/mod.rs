use crate::primitives::DecodedImage;
use crate::render::{ImagePaint, LinePaint, Paint, PointPaint, PrimitiveId};
use galileo_types::cartesian::impls::point::Point2d;
use galileo_types::cartesian::traits::cartesian_point::CartesianPoint3d;
use galileo_types::contour::Contour;
use galileo_types::polygon::Polygon;
use num_traits::AsPrimitive;
use tessellating::TessellatingRenderBundle;

pub mod tessellating;

#[non_exhaustive]
pub enum RenderBundle {
    Tessellating(TessellatingRenderBundle),
}

impl RenderBundle {
    pub fn add_image(
        &mut self,
        image: DecodedImage,
        vertices: [Point2d; 4],
        paint: ImagePaint,
    ) -> PrimitiveId {
        match self {
            RenderBundle::Tessellating(inner) => inner.add_image(image, vertices, paint),
        }
    }

    pub fn add_point<N, P>(&mut self, point: &P, paint: PointPaint) -> PrimitiveId
    where
        N: AsPrimitive<f32>,
        P: CartesianPoint3d<Num = N>,
    {
        match self {
            RenderBundle::Tessellating(inner) => inner.add_point(point, paint),
        }
    }

    pub fn add_line<N, P, C>(&mut self, line: &C, paint: LinePaint, resolution: f64) -> PrimitiveId
    where
        N: AsPrimitive<f32>,
        P: CartesianPoint3d<Num = N>,
        C: Contour<Point = P>,
    {
        match self {
            RenderBundle::Tessellating(inner) => inner.add_line(line, paint, resolution),
        }
    }

    pub fn add_polygon<N, P, Poly>(
        &mut self,
        polygon: &Poly,
        paint: Paint,
        resolution: f64,
    ) -> PrimitiveId
    where
        N: AsPrimitive<f32>,
        P: CartesianPoint3d<Num = N>,
        Poly: Polygon,
        Poly::Contour: Contour<Point = P>,
    {
        match self {
            RenderBundle::Tessellating(inner) => inner.add_polygon(polygon, paint, resolution),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            RenderBundle::Tessellating(inner) => inner.is_empty(),
        }
    }
}