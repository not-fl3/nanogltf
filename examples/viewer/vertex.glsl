attribute vec3 in_position; // [[attribute(0)]]
attribute vec2 in_uv; // [[attribute(1)]];
attribute vec3 in_normal; // [[attribute(2)]];

varying vec2 out_uv; // [[user(locn0)]];
varying vec3 out_pos; // [[user(locn1)]];
varying vec3 out_normal; // [[user(locn2)]];

uniform mat4 Model;
uniform mat4 ModelInverse;
uniform mat4 Projection;

mat3 transpose(mat3 m) {
    return mat3(vec3(m[0].x, m[1].x, m[2].x),
                vec3(m[0].y, m[1].y, m[2].y),
                vec3(m[0].z, m[1].z, m[2].z));
}

void main() {
    gl_Position = Projection * Model * vec4(in_position, 1);
    out_uv = in_uv;
    out_normal = transpose(mat3(ModelInverse)) * in_normal;
    out_pos = vec3(Model * vec4(in_position, 1.0));
}
