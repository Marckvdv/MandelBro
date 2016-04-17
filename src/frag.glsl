#version 140

uniform vec2 res;
uniform int MAX_ITER;
uniform vec2 center;
uniform float zoom;

out vec4 color;

vec3 rgb2hsv(vec3 c) {
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
    vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

int iterateMandelbrot(vec2 pos) {
	float r = 0.0;
	float i = 0.0;

	int iter = 0;
	while(r*r + i*i <= 4.0 && iter < MAX_ITER) {
		float nextR = r*r - i*i + pos.x;
		float nextI = 2.0*r*i + pos.y;

		r = nextR;
		i = nextI;

		++iter;
	}

	return iter;
}

void main() {
	float aspect = res.y / res.x;
	vec2 uv = gl_FragCoord.xy / res;
	uv -= 0.5;
	uv.y *= aspect;

	vec2 pos = uv / zoom + center;

	int iter = iterateMandelbrot(pos);
	
	vec3 rgb = hsv2rgb(vec3((iter % 100) / 100.0, 1.0, 1.0));

	color = vec4(rgb, 1.0);
}

