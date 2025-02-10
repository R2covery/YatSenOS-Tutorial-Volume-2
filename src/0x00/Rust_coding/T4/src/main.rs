use std::f64::consts::PI;

// 定义 Shape 枚举
enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

// 为 Shape 枚举实现 area 方法
impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Circle { radius } => PI * radius * radius,
        }
    }
}

// 测试代码
#[test]
fn test_area() {
    let rectangle = Shape::Rectangle {
        width: 10.0,
        height: 20.0,
    };
    let circle = Shape::Circle { radius: 10.0 };

    assert_eq!(rectangle.area(), 200.0);
    assert_eq!(circle.area(), 314.1592653589793);
}

fn main() {
    // 示例用法
    let rectangle = Shape::Rectangle {
        width: 10.0,
        height: 20.0,
    };
    let circle = Shape::Circle { radius: 10.0 };

    println!("Rectangle area: {}", rectangle.area());
    println!("Circle area: {}", circle.area());
}