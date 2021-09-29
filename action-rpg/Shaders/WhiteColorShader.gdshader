shader_type canvas_item;

uniform bool active = false;

void fragment() {
	COLOR = active ? vec4(1.0, 1.0, 1.0, texture(TEXTURE, UV).a) : texture(TEXTURE, UV);
}