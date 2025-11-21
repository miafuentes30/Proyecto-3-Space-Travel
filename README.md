# Proyecto 3: Space Travel

Simulación del sistema solar implementada en Rust con Raylib.

Video para visualizar el sistema solar: 
https://www.canva.com/design/DAG5SXGn9YY/ATHc6pbOUf_atuLHxc5x3A/edit?utm_content=DAG5SXGn9YY&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton

## Descripción

Este proyecto recrea el sistema solar completo con:
- **1 estrella (Sol)** con plasma turbulento animado
- **8 planetas principales** (Mercurio a Neptuno) con shaders especializados
- **4 lunas** (Luna de la Tierra, Fobos y Deimos de Marte, Titán de Saturno)
- Movimiento orbital realista en el plano eclíptico
- Física de colisión para evitar atravesar cuerpos celestes
- Sistema de warp instantáneo a cualquier planeta
- Nave espacial personalizada que sigue a la cámara
- Skybox con textura de estrellas fijo en el horizonte

## Características Técnicas

### Sistemas Implementados
- **Cámara 3D completa**: Movimiento en 6 direcciones + rotación horizontal
- **Órbitas renderizadas**: Trayectorias circulares visibles para planetas
- **Sistema de colisión**: Previene que la cámara atraviese planetas o el sol
- **Warp animado**: Transiciones suaves con interpolación ease-in-out
- **Nave modelada**: Modelo OBJ personalizado que sigue la cámara con orientación dinámica
- **Skybox estático**: Esfera invertida con textura de estrellas anclada al origen

## Controles

### Movimiento de Cámara
- **W**: Avanzar (en el plano eclíptico)
- **S**: Retroceder
- **A**: Mover a la izquierda
- **D**: Mover a la derecha
- **Espacio**: Subir (eje Y positivo)
- **Shift Izquierdo**: Bajar (eje Y negativo)
- **Flecha Izquierda**: Rotar cámara hacia la izquierda
- **Flecha Derecha**: Rotar cámara hacia la derecha

### Warps Instantáneos
- **0**: Warp al Sol
- **1**: Warp a Mercurio
- **2**: Warp a Venus
- **3**: Warp a la Tierra
- **4**: Warp a Marte
- **5**: Warp a Júpiter
- **6**: Warp a Saturno
- **7**: Warp a Urano
- **8**: Warp a Neptuno

### Visualización
- **O**: Toggle órbitas planetarias (mostrar/ocultar)
- **I**: Toggle información en pantalla (controles y posición)
- **V**: Toggle modo demo de órbita de la nave
- **B**: Vista cenital (top-down del sistema)
- **B R**: Regresar a posición inicial (reset cámara)

## Compilación y Ejecución

### Requisitos Previos
- Rust (versión 1.70 o superior)
- Raylib instalado en el sistema o configurado en Cargo.toml

### Modo Release (Recomendado)
```powershell
cargo build --release
cargo run --release
```

### Modo Debug (Desarrollo)
```powershell
cargo run
```
## Estructura del Proyecto

```
Proyecto-3-Space-Travel/
├── src/
│   ├── main.rs              # Loop principal y creación del sistema solar
│   ├── camera.rs            # Controlador de cámara 3D con warps
│   ├── celestial_body.rs    # Estructura y lógica de planetas/lunas
│   ├── spaceship.rs         # Nave espacial con órbita demo
│   ├── collision.rs         # Sistema de detección y resolución de colisiones
│   ├── orbit.rs             # Renderizado de órbitas circulares
│   ├── skybox.rs            # Esfera celeste con estrellas
│   ├── warp_effect.rs       # Efectos de transición de warp
│   ├── shader.rs            # Shaders procedurales CPU y generación de texturas
│   └── renderer.rs          # Renderer software alternativo
├── assets/
│   ├── models/
│   │   ├── sphere.obj       # Modelo esférico para planetas
│   │   └── nave.obj         # Modelo 3D de la nave espacial
│   └── textures/
│       └── skybox.png       # Textura del skybox con estrellas
├── Cargo.toml
└── README.md
```

## Notas Técnicas

### Generación de Texturas
- Las texturas se generan pixel por pixel en CPU usando algoritmos de ruido Perlin/FBM
- Se aplican directamente a materiales de modelos 3D sin archivos intermedios
- Implementación de `seam_noise` para eliminar artefactos de costura vertical
- Cache permanente: texturas se generan solo al inicio para optimizar performance

### Sistema de Órbitas
- Cada planeta tiene velocidad orbital y radio definidos
- Lunas orbitan alrededor de sus planetas padres (índice de referencia)
- Movimiento circular simple en el plano XZ (Y constante)

### Performance
- Generación única de texturas al inicio (aprox. 1-2 segundos)
- Modelos reutilizados con materiales dinámicos
- Sin regeneración por frame (texturas estáticas)
- Target: 60 FPS en hardware moderno
