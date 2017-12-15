use servo::gl::{Gl, GlType, __GLsync};
use std::os::raw::c_void;
use std::rc::Rc;

pub struct LogGl {
  gl: Rc<Gl>,
}
impl LogGl {
  pub fn new(gl: Rc<Gl>) -> Rc<LogGl> {
    Rc::new(LogGl {
      gl: gl
    })
  }
}

impl Gl for LogGl {
    fn get_type(&self) -> GlType {
        info!("get_type");
        self.gl.get_type()
    }
    fn buffer_data_untyped(
        &self, 
        target: u32, 
        size: isize, 
        data: *const c_void, 
        usage: u32
    ) {
        info!("buffer_data_untyped(target: {}, size: {}, data: {:?}, usage: {})", target, size, data, usage);
        self.gl.buffer_data_untyped(target, size, data, usage)
    }
    fn buffer_sub_data_untyped(
        &self, 
        target: u32, 
        offset: isize, 
        size: isize, 
        data: *const c_void
    ) {
        info!("buffer_sub_data_untyped(target: {}, offset: {}, size: {}, data: {:?})", target, offset, size, data);
        self.gl.buffer_sub_data_untyped(target, offset, size, data)
    }
    fn tex_buffer(&self, target: u32, internal_format: u32, buffer: u32) {
        info!("tex_buffer(target: {}, internal_format: {}, buffer: {})", target, internal_format, buffer);
        self.gl.tex_buffer(target, internal_format, buffer)
    }
    fn shader_source(&self, shader: u32, strings: &[&[u8]]) {
        info!("shader_source(shader: {}, strings: (array len {}))", shader, strings.len());
        self.gl.shader_source(shader, strings)
    }
    fn read_buffer(&self, mode: u32) {
        info!("read_buffer(mode: {})", mode);
        self.gl.read_buffer(mode)
    }
    fn read_pixels_into_buffer(
        &self, 
        x: i32, 
        y: i32, 
        width: i32, 
        height: i32, 
        format: u32, 
        pixel_type: u32, 
        dst_buffer: &mut [u8]
    ) {
        info!("read_pixels_into_buffer(x: {}, y: {}, width: {}, height: {}, format: {}, pixel_type: {}, dst_buffer: ({}))", x, y, width, height, format, pixel_type, dst_buffer.len());
        self.gl.read_pixels_into_buffer(x, y, width, height, format, pixel_type, dst_buffer)
    }
    fn read_pixels(
        &self, 
        x: i32, 
        y: i32, 
        width: i32, 
        height: i32, 
        format: u32, 
        pixel_type: u32
    ) -> Vec<u8> {
        info!("read_pixels(x: {}, y: {}, width: {}, height: {}, format: {}, pixel_type: {})", x, y, width, height, format, pixel_type);
        self.gl.read_pixels(x, y, width, height, format, pixel_type)
    }
    fn sample_coverage(&self, value: f32, invert: bool) {
        info!("sample_coverage(value: {}, invert: {})", value, invert);
        self.gl.sample_coverage(value, invert)
    }
    fn polygon_offset(&self, factor: f32, units: f32) {
        info!("polygon_offset(factor: {}, units: {})", factor, units);
        self.gl.polygon_offset(factor, units)
    }
    fn pixel_store_i(&self, name: u32, param: i32) {
        info!("pixel_store_i(name: {}, param: {})", name, param);
        self.gl.pixel_store_i(name, param)
    }
    fn gen_buffers(&self, n: i32) -> Vec<u32> {
        info!("gen_buffers(n: {})", n);
        self.gl.gen_buffers(n)
    }
    fn gen_renderbuffers(&self, n: i32) -> Vec<u32> {
        info!("gen_renderbuffers(n: {})", n);
        self.gl.gen_renderbuffers(n)
    }
    fn gen_framebuffers(&self, n: i32) -> Vec<u32> {
        info!("gen_framebuffers(n: {})", n);
        self.gl.gen_framebuffers(n)
    }
    fn gen_textures(&self, n: i32) -> Vec<u32> {
        info!("gen_textures(n: {})", n);
        self.gl.gen_textures(n)
    }
    fn gen_vertex_arrays(&self, n: i32) -> Vec<u32> {
        info!("gen_vertex_arrays(n: {})", n);
        self.gl.gen_vertex_arrays(n)
    }
    fn gen_queries(&self, n: i32) -> Vec<u32> {
        info!("gen_queries(n: {})", n);
        self.gl.gen_queries(n)
    }
    fn begin_query(&self, target: u32, id: u32) {
        info!("begin_query(target: {}, id: {})", target, id);
        self.gl.begin_query(target, id)
    }
    fn end_query(&self, target: u32) {
        info!("end_query(target: {})", target);
        self.gl.end_query(target)
    }
    fn query_counter(&self, id: u32, target: u32) {
        info!("query_counter(id: {}, target: {})", id, target);
        self.gl.query_counter(id, target)
    }
    fn get_query_object_iv(&self, id: u32, pname: u32) -> i32 {
        info!("get_query_object_iv(id: {}, pname: {})", id, pname);
        self.gl.get_query_object_iv(id, pname)
    }
    fn get_query_object_uiv(&self, id: u32, pname: u32) -> u32 {
        info!("get_query_object_uiv(id: {}, pname: {})", id, pname);
        self.gl.get_query_object_uiv(id, pname)
    }
    fn get_query_object_i64v(&self, id: u32, pname: u32) -> i64 {
        info!("get_query_object_i64v(id: {}, pname: {})", id, pname);
        self.gl.get_query_object_i64v(id, pname)
    }
    fn get_query_object_ui64v(&self, id: u32, pname: u32) -> u64 {
        info!("get_query_object_ui64v(id: {}, pname: {})", id, pname);
        self.gl.get_query_object_ui64v(id, pname)
    }
    fn delete_queries(&self, queries: &[u32]) {
        info!("delete_queries(queries: (array len {}))", queries.len());
        self.gl.delete_queries(queries)
    }
    fn delete_vertex_arrays(&self, vertex_arrays: &[u32]) {
        info!("delete_vertex_arrays(vertex_arrays: (array len {}))", vertex_arrays.len());
        self.gl.delete_vertex_arrays(vertex_arrays)
    }
    fn delete_buffers(&self, buffers: &[u32]) {
        info!("delete_buffers(buffers: ({}))", buffers.len());
        self.gl.delete_buffers(buffers)
    }
    fn delete_renderbuffers(&self, renderbuffers: &[u32]) {
        info!("delete_renderbuffers(renderbuffers: ({}))", renderbuffers.len());
        self.gl.delete_renderbuffers(renderbuffers)
    }
    fn delete_framebuffers(&self, framebuffers: &[u32]) {
        info!("delete_framebuffers(framebuffers: ({}))", framebuffers.len());
        self.gl.delete_framebuffers(framebuffers)
    }
    fn delete_textures(&self, textures: &[u32]) {
        info!("delete_textures(textures: ({}))", textures.len());
        self.gl.delete_textures(textures)
    }
    fn framebuffer_renderbuffer(
        &self, 
        target: u32, 
        attachment: u32, 
        renderbuffertarget: u32, 
        renderbuffer: u32
    ) {
        info!("framebuffer_renderbuffer(target: {}, attachment: {}, renderbuffertarget: {}, renderbuffer: {})", target, attachment, renderbuffertarget, renderbuffer);
        self.gl.framebuffer_renderbuffer(target, attachment, renderbuffertarget, renderbuffer)
    }
    fn renderbuffer_storage(
        &self, 
        target: u32, 
        internalformat: u32, 
        width: i32, 
        height: i32
    ) {
        info!("renderbuffer_storage(target: {}, internalformat: {}, width: {}, height: {})", target, internalformat, width, height);
        self.gl.renderbuffer_storage(target, internalformat, width, height)
    }
    fn depth_func(&self, func: u32) {
        info!("depth_func(func: {})", func);
        self.gl.depth_func(func)
    }
    fn active_texture(&self, texture: u32) {
        info!("active_texture(texture: {})", texture);
        self.gl.active_texture(texture)
    }
    fn attach_shader(&self, program: u32, shader: u32) {
        info!("attach_shader(program: {}, shader: {})", program, shader);
        self.gl.attach_shader(program, shader)
    }
    fn bind_attrib_location(&self, program: u32, index: u32, name: &str) {
        info!("bind_attrib_location(program: {}, index: {}, name: {})", program, index, name);
        self.gl.bind_attrib_location(program, index, name)
    }
    fn get_uniform_block_index(&self, program: u32, name: &str) -> u32 {
        info!("get_uniform_block_index(program: {}, name: {})", program, name);
        self.gl.get_uniform_block_index(program, name)
    }
    fn get_uniform_indices(&self, program: u32, names: &[&str]) -> Vec<u32> {
        info!("get_uniform_indices(program: {}, names: ({}))", program, names.len());
        self.gl.get_uniform_indices(program, names)
    }
    fn bind_buffer_base(&self, target: u32, index: u32, buffer: u32) {
        info!("bind_buffer_base(target: {}, index: {}, buffer: {})", target, index, buffer);
        self.gl.bind_buffer_base(target, index, buffer)
    }
    fn bind_buffer_range(
        &self, 
        target: u32, 
        index: u32, 
        buffer: u32, 
        offset: isize, 
        size: isize
    ) {
        info!("bind_buffer_range(target: {}, index: {}, buffer: {}, offset: {}, size: {})", target, index, buffer, offset, size);
        self.gl.bind_buffer_range(target, index, buffer, offset, size)
    }
    fn uniform_block_binding(
        &self, 
        program: u32, 
        uniform_block_index: u32, 
        uniform_block_binding: u32
    ) {
        info!("uniform_block_binding(program: {}, uniform_block_index: {}, uniform_block_binding: {})", program, uniform_block_index, uniform_block_binding);
        self.gl.uniform_block_binding(program, uniform_block_index, uniform_block_binding)
    }
    fn bind_buffer(&self, target: u32, buffer: u32) {
        info!("bind_buffer(target: {}, buffer: {})", target, buffer);
        self.gl.bind_buffer(target, buffer)
    }
    fn bind_vertex_array(&self, vao: u32) {
        info!("bind_vertex_array(vao: {})", vao);
        self.gl.bind_vertex_array(vao)
    }
    fn bind_renderbuffer(&self, target: u32, renderbuffer: u32) {
        info!("bind_renderbuffer(target: {}, renderbuffer: {})", target, renderbuffer);
        self.gl.bind_renderbuffer(target, renderbuffer)
    }
    fn bind_framebuffer(&self, target: u32, framebuffer: u32) {
        info!("bind_framebuffer(target: {}, framebuffer: {})", target, framebuffer);
        self.gl.bind_framebuffer(target, framebuffer)
    }
    fn bind_texture(&self, target: u32, texture: u32) {
        info!("bind_texture(target: {}, texture: {})", target, texture);
        self.gl.bind_texture(target, texture)
    }
    fn tex_image_2d(
        &self, 
        target: u32, 
        level: i32, 
        internal_format: i32, 
        width: i32, 
        height: i32, 
        border: i32, 
        format: u32, 
        ty: u32, 
        opt_data: Option<&[u8]>
    ) {
        info!("tex_image_2d(target: {}, level: {}, internal_format: {}, width: {}, height: {}, border: {}, format: {}, ty: {}, opt_data: ({}))", target, level, internal_format, width, height, border, format, ty, opt_data.as_ref().map(|x| x.len()).unwrap_or(0));
        self.gl.tex_image_2d(target, level, internal_format, width, height, border, format, ty, opt_data)
    }
    fn compressed_tex_image_2d(
        &self, 
        target: u32, 
        level: i32, 
        internal_format: u32, 
        width: i32, 
        height: i32, 
        border: i32, 
        data: &[u8]
    ) {
        info!("compressed_tex_image_2d(target: {}, level: {}, internal_format: {}, width: {}, height: {}, border: {}, data: ({}))", target, level, internal_format, width, height, border, data.len());
        self.gl.compressed_tex_image_2d(target, level, internal_format, width, height, border, data)
    }
    fn compressed_tex_sub_image_2d(
        &self, 
        target: u32, 
        level: i32, 
        xoffset: i32, 
        yoffset: i32, 
        width: i32, 
        height: i32, 
        format: u32, 
        data: &[u8]
    ) {
        info!("compressed_tex_sub_image_2d(target: {}, level: {}, xoffset: {}, yoffset: {}, width: {}, height: {}, format: {}, data: ({}))", target, level, xoffset, yoffset, width, height, format, data.len());
        self.gl.compressed_tex_sub_image_2d(target, level, xoffset, yoffset, width, height, format, data)
    }
    fn tex_image_3d(
        &self, 
        target: u32, 
        level: i32, 
        internal_format: i32, 
        width: i32, 
        height: i32, 
        depth: i32, 
        border: i32, 
        format: u32, 
        ty: u32, 
        opt_data: Option<&[u8]>
    ) {
        info!("tex_image_3d target: {}, level: {}, internal_format: {}, width: {}, height: {}, depth: {}, border: {}, format: {}, ty: {}, opt_data: {}", target, level, internal_format, width, height, depth, border, format, ty, opt_data.as_ref().map(|x| x.len()).unwrap_or(0));
        self.gl.tex_image_3d(target, level, internal_format, width, height, depth, border, format, ty, opt_data)
    }
    fn copy_tex_image_2d(
        &self, 
        target: u32, 
        level: i32, 
        internal_format: u32, 
        x: i32, 
        y: i32, 
        width: i32, 
        height: i32, 
        border: i32
    ) {
        info!("copy_tex_image_2d(target: {}, level: {}, internal_format: {}, x: {}, y: {}, width: {}, height: {}, border: {})", target, level, internal_format, x, y, width, height, border);
        self.gl.copy_tex_image_2d(target, level, internal_format, x, y, width, height, border)
    }
    fn copy_tex_sub_image_2d(
        &self, 
        target: u32, 
        level: i32, 
        xoffset: i32, 
        yoffset: i32, 
        x: i32, 
        y: i32, 
        width: i32, 
        height: i32
    ) {
        info!("copy_tex_sub_image_2d(target: {}, level: {}, xoffset: {}, yoffset: {}, x: {}, y: {}, width: {}, height: {})", target, level, xoffset, yoffset, x, y, width, height);
        self.gl.copy_tex_sub_image_2d(target, level, xoffset, yoffset, x, y, width, height)
    }
    fn copy_tex_sub_image_3d(
        &self, 
        target: u32, 
        level: i32, 
        xoffset: i32, 
        yoffset: i32, 
        zoffset: i32, 
        x: i32, 
        y: i32, 
        width: i32, 
        height: i32
    ) {
        info!("copy_tex_sub_image_3d(target: {}, level: {}, xoffset: {}, yoffset: {}, zoffset: {}, x: {}, y: {}, width: {}, height: {})", target, level, xoffset, yoffset, zoffset, x, y, width, height);
        self.gl.copy_tex_sub_image_3d(target, level, xoffset, yoffset, zoffset, x, y, width, height)
    }
    fn tex_sub_image_2d(
        &self, 
        target: u32, 
        level: i32, 
        xoffset: i32, 
        yoffset: i32, 
        width: i32, 
        height: i32, 
        format: u32, 
        ty: u32, 
        data: &[u8]
    ) {
        info!("tex_sub_image_2d(target: {}, level: {}, xoffset: {}, yoffset: {}, width: {}, height: {}, format: {}, ty: {}, data: ({}))", target, level, xoffset, yoffset, width, height, format, ty, data.len());
        self.gl.tex_sub_image_2d(target, level, xoffset, yoffset, width, height, format, ty, data)
    }
    fn tex_sub_image_2d_pbo(
        &self, 
        target: u32, 
        level: i32, 
        xoffset: i32, 
        yoffset: i32, 
        width: i32, 
        height: i32, 
        format: u32, 
        ty: u32, 
        offset: usize
    ) {
        info!("tex_sub_image_2d_pbo(target: {}, level: {}, xoffset: {}, yoffset: {}, width: {}, height: {}, format: {}, ty: {}, offset: {})", target, level, xoffset, yoffset, width, height, format, ty, offset);
        self.gl.tex_sub_image_2d_pbo(target, level, xoffset, yoffset, width, height, format, ty, offset)
    }
    fn tex_sub_image_3d(
        &self, 
        target: u32, 
        level: i32, 
        xoffset: i32, 
        yoffset: i32, 
        zoffset: i32, 
        width: i32, 
        height: i32, 
        depth: i32, 
        format: u32, 
        ty: u32, 
        data: &[u8]
    ) {
        info!("tex_sub_image_3d target: {}, level: {}, xoffset: {}, yoffset: {}, zoffset: {}, width: {}, height: {}, depth: {}, format: {}, ty: {}, data: ({})", target, level, xoffset, yoffset, zoffset, width, height, depth, format, ty, data.len());
        self.gl.tex_sub_image_3d(target, level, xoffset, yoffset, zoffset, width, height, depth, format, ty, data)
    }
    fn tex_sub_image_3d_pbo(
        &self, 
        target: u32, 
        level: i32, 
        xoffset: i32, 
        yoffset: i32, 
        zoffset: i32, 
        width: i32, 
        height: i32, 
        depth: i32, 
        format: u32, 
        ty: u32, 
        offset: usize
    ) {
        info!("tex_sub_image_3d_pbo target: {}, level: {}, xoffset: {}, yoffset: {}, zoffset: {}, width: {}, height: {}, depth: {}, format: {}, ty: {}, offset: {}", target, level, xoffset, yoffset, zoffset, width, height, depth, format, ty, offset);
        self.gl.tex_sub_image_3d_pbo(target, level, xoffset, yoffset, zoffset, width, height, depth, format, ty, offset)
    }
    fn get_integer_v(&self, name: u32) -> i32 {
        info!("get_integer_v(name: {})", name);
        self.gl.get_integer_v(name)
    }
    fn get_integer_64v(&self, name: u32) -> i64 {
        info!("get_integer_64v(name: {})", name);
        self.gl.get_integer_64v(name)
    }
    fn get_integer_iv(&self, name: u32, index: u32) -> i32 {
        info!("get_integer_iv(name: {}, index: {})", name, index);
        self.gl.get_integer_iv(name, index)
    }
    fn get_integer_64iv(&self, name: u32, index: u32) -> i64 {
        info!("get_integer_64iv(name: {}, index: {})", name, index);
        self.gl.get_integer_64iv(name, index)
    }
    fn get_boolean_v(&self, name: u32) -> u8 {
        info!("get_boolean_v(name: {})", name);
        self.gl.get_boolean_v(name)
    }
    fn get_float_v(&self, name: u32) -> f32 {
        info!("get_float_v(name: {})", name);
        self.gl.get_float_v(name)
    }
    fn tex_parameter_i(&self, target: u32, pname: u32, param: i32) {
        info!("tex_parameter_i(target: {}, pname: {}, param: {})", target, pname, param);
        self.gl.tex_parameter_i(target, pname, param)
    }
    fn tex_parameter_f(&self, target: u32, pname: u32, param: f32) {
        info!("tex_parameter_f(target: {}, pname: {}, param: {})", target, pname, param);
        self.gl.tex_parameter_f(target, pname, param)
    }
    fn framebuffer_texture_2d(
        &self, 
        target: u32, 
        attachment: u32, 
        textarget: u32, 
        texture: u32, 
        level: i32
    ) {
        info!("framebuffer_texture_2d(target: {}, attachment: {}, textarget: {}, texture: {}, level: {})", target, attachment, textarget, texture, level);
        self.gl.framebuffer_texture_2d(target, attachment, textarget, texture, level)
    }
    fn framebuffer_texture_layer(
        &self, 
        target: u32, 
        attachment: u32, 
        texture: u32, 
        level: i32, 
        layer: i32
    ) {
        info!("framebuffer_texture_layer(target: {}, attachment: {}, texture: {}, level: {}, layer: {})", target, attachment, texture, level, layer);
        self.gl.framebuffer_texture_layer(target, attachment, texture, level, layer)
    }
    fn blit_framebuffer(
        &self, 
        src_x0: i32, 
        src_y0: i32, 
        src_x1: i32, 
        src_y1: i32, 
        dst_x0: i32, 
        dst_y0: i32, 
        dst_x1: i32, 
        dst_y1: i32, 
        mask: u32, 
        filter: u32
    ) {
        info!("blit_framebuffer src_x0: {}, src_y0: {}, src_x1: {}, src_y1: {}, dst_x0: {}, dst_y0: {}, dst_x1: {}, dst_y1: {}, mask: {}, filter: {}", src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter);
        self.gl.blit_framebuffer(src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter)
    }
    fn vertex_attrib_4f(&self, index: u32, x: f32, y: f32, z: f32, w: f32) {
        info!("vertex_attrib_4f(index: {}, x: {}, y: {}, z: {}, w: {})", index, x, y, z, w);
        self.gl.vertex_attrib_4f(index, x, y, z, w)
    }
    fn vertex_attrib_pointer_f32(
        &self, 
        index: u32, 
        size: i32, 
        normalized: bool, 
        stride: i32, 
        offset: u32
    ) {
        info!("vertex_attrib_pointer_f32(index: {}, size: {}, normalized: {}, stride: {}, offset: {})", index, size, normalized, stride, offset);
        self.gl.vertex_attrib_pointer_f32(index, size, normalized, stride, offset)
    }
    fn vertex_attrib_pointer(
        &self, 
        index: u32, 
        size: i32, 
        type_: u32, 
        normalized: bool, 
        stride: i32, 
        offset: u32
    ) {
        info!("vertex_attrib_pointer(index: {}, size: {}, type_: {}, normalized: {}, stride: {}, offset: {})", index, size, type_, normalized, stride, offset);
        self.gl.vertex_attrib_pointer(index, size, type_, normalized, stride, offset)
    }
    fn vertex_attrib_i_pointer(
        &self, 
        index: u32, 
        size: i32, 
        type_: u32, 
        stride: i32, 
        offset: u32
    ) {
        info!("vertex_attrib_i_pointer(index: {}, size: {}, type_: {}, stride: {}, offset: {})", index, size, type_, stride, offset);
        self.gl.vertex_attrib_i_pointer(index, size, type_, stride, offset)
    }
    fn vertex_attrib_divisor(&self, index: u32, divisor: u32) {
        info!("vertex_attrib_divisor(index: {}, divisor: {})", index, divisor);
        self.gl.vertex_attrib_divisor(index, divisor)
    }
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        info!("viewport(x: {}, y: {}, width: {}, height: {})", x, y, width, height);
        self.gl.viewport(x, y, width, height)
    }
    fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        info!("scissor(x: {}, y: {}, width: {}, height: {})", x, y, width, height);
        self.gl.scissor(x, y, width, height)
    }
    fn line_width(&self, width: f32) {
        info!("line_width(width: {})", width);
        self.gl.line_width(width)
    }
    fn use_program(&self, program: u32) {
        info!("use_program(program: {})", program);
        self.gl.use_program(program)
    }
    fn validate_program(&self, program: u32) {
        info!("validate_program(program: {})", program);
        self.gl.validate_program(program)
    }
    fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        info!("draw_arrays(mode: {}, first: {}, count: {})", mode, first, count);
        self.gl.draw_arrays(mode, first, count)
    }
    fn draw_arrays_instanced(
        &self, 
        mode: u32, 
        first: i32, 
        count: i32, 
        primcount: i32
    ) {
        info!("draw_arrays_instanced(mode: {}, first: {}, count: {}, primcount: {})", mode, first, count, primcount);
        self.gl.draw_arrays_instanced(mode, first, count, primcount)
    }
    fn draw_elements(
        &self, 
        mode: u32, 
        count: i32, 
        element_type: u32, 
        indices_offset: u32
    ) {
        info!("draw_elements(mode: {}, count: {}, element_type: {}, indices_offset: {})", mode, count, element_type, indices_offset);
        self.gl.draw_elements(mode, count, element_type, indices_offset)
    }
    fn draw_elements_instanced(
        &self, 
        mode: u32, 
        count: i32, 
        element_type: u32, 
        indices_offset: u32, 
        primcount: i32
    ) {
        info!("draw_elements_instanced(mode: {}, count: {}, element_type: {}, indices_offset: {}, primcount: {})", mode, count, element_type, indices_offset, primcount);
        self.gl.draw_elements_instanced(mode, count, element_type, indices_offset, primcount)
    }
    fn blend_color(&self, r: f32, g: f32, b: f32, a: f32) {
        info!("blend_color(r: {}, g: {}, b: {}, a: {})", r, g, b, a);
        self.gl.blend_color(r, g, b, a)
    }
    fn blend_func(&self, sfactor: u32, dfactor: u32) {
        info!("blend_func(sfactor: {}, dfactor: {})", sfactor, dfactor);
        self.gl.blend_func(sfactor, dfactor)
    }
    fn blend_func_separate(
        &self, 
        src_rgb: u32, 
        dest_rgb: u32, 
        src_alpha: u32, 
        dest_alpha: u32
    ) {
        info!("blend_func_separate(src_rgb: {}, dest_rgb: {}, src_alpha: {}, dest_alpha: {})", src_rgb, dest_rgb, src_alpha, dest_alpha);
        self.gl.blend_func_separate(src_rgb, dest_rgb, src_alpha, dest_alpha)
    }
    fn blend_equation(&self, mode: u32) {
        info!("blend_equation(mode: {})", mode);
        self.gl.blend_equation(mode)
    }
    fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32) {
        info!("blend_equation_separate(mode_rgb: {}, mode_alpha: {})", mode_rgb, mode_alpha);
        self.gl.blend_equation_separate(mode_rgb, mode_alpha)
    }
    fn color_mask(&self, r: bool, g: bool, b: bool, a: bool) {
        info!("color_mask(r: {}, g: {}, b: {}, a: {})", r, g, b, a);
        self.gl.color_mask(r, g, b, a)
    }
    fn cull_face(&self, mode: u32) {
        info!("cull_face(mode: {})", mode);
        self.gl.cull_face(mode)
    }
    fn front_face(&self, mode: u32) {
        info!("front_face(mode: {})", mode);
        self.gl.front_face(mode)
    }
    fn enable(&self, cap: u32) {
        info!("enable(cap: {})", cap);
        self.gl.enable(cap)
    }
    fn disable(&self, cap: u32) {
        info!("disable(cap: {})", cap);
        self.gl.disable(cap)
    }
    fn hint(&self, param_name: u32, param_val: u32) {
        info!("hint(param_name: {}, param_val: {})", param_name, param_val);
        self.gl.hint(param_name, param_val)
    }
    fn is_enabled(&self, cap: u32) -> u8 {
        info!("is_enabled(cap: {})", cap);
        self.gl.is_enabled(cap)
    }
    fn is_shader(&self, shader: u32) -> u8 {
        info!("is_shader(shader: {})", shader);
        self.gl.is_shader(shader)
    }
    fn is_texture(&self, texture: u32) -> u8 {
        info!("is_texture(texture: {})", texture);
        self.gl.is_texture(texture)
    }
    fn is_framebuffer(&self, framebuffer: u32) -> u8 {
        info!("is_framebuffer(framebuffer: {})", framebuffer);
        self.gl.is_framebuffer(framebuffer)
    }
    fn is_renderbuffer(&self, renderbuffer: u32) -> u8 {
        info!("is_renderbuffer(renderbuffer: {})", renderbuffer);
        self.gl.is_renderbuffer(renderbuffer)
    }
    fn check_frame_buffer_status(&self, target: u32) -> u32 {
        info!("check_frame_buffer_status(target: {})", target);
        self.gl.check_frame_buffer_status(target)
    }
    fn enable_vertex_attrib_array(&self, index: u32) {
        info!("enable_vertex_attrib_array(index: {})", index);
        self.gl.enable_vertex_attrib_array(index)
    }
    fn disable_vertex_attrib_array(&self, index: u32) {
        info!("disable_vertex_attrib_array(index: {})", index);
        self.gl.disable_vertex_attrib_array(index)
    }
    fn uniform_1f(&self, location: i32, v0: f32) {
        info!("uniform_1f(location: {}, v0: {})", location, v0);
        self.gl.uniform_1f(location, v0)
    }
    fn uniform_1fv(&self, location: i32, values: &[f32]) {
        info!("uniform_1fv(location: {}, values: ({}))", location, values.len());
        self.gl.uniform_1fv(location, values)
    }
    fn uniform_1i(&self, location: i32, v0: i32) {
        info!("uniform_1i(location: {}, v0: {})", location, v0);
        self.gl.uniform_1i(location, v0)
    }
    fn uniform_1iv(&self, location: i32, values: &[i32]) {
        info!("uniform_1iv(location: {}, values: ({}))", location, values.len());
        self.gl.uniform_1iv(location, values)
    }
    fn uniform_1ui(&self, location: i32, v0: u32) {
        info!("uniform_1ui(location: {}, v0: {})", location, v0);
        self.gl.uniform_1ui(location, v0)
    }
    fn uniform_2f(&self, location: i32, v0: f32, v1: f32) {
        info!("uniform_2f(location: {}, v0: {}, v1: {})", location, v0, v1);
        self.gl.uniform_2f(location, v0, v1)
    }
    fn uniform_2fv(&self, location: i32, values: &[f32]) {
        info!("uniform_2fv(location: {}, values: ({}))", location, values.len());
        self.gl.uniform_2fv(location, values)
    }
    fn uniform_2i(&self, location: i32, v0: i32, v1: i32) {
        info!("uniform_2i(location: {}, v0: {}, v1: {})", location, v0, v1);
        self.gl.uniform_2i(location, v0, v1)
    }
    fn uniform_2iv(&self, location: i32, values: &[i32]) {
        info!("uniform_2iv(location: {}, values: ({}))", location, values.len());
        self.gl.uniform_2iv(location, values)
    }
    fn uniform_2ui(&self, location: i32, v0: u32, v1: u32) {
        info!("uniform_2ui(location: {}, v0: {}, v1: {})", location, v0, v1);
        self.gl.uniform_2ui(location, v0, v1)
    }
    fn uniform_3f(&self, location: i32, v0: f32, v1: f32, v2: f32) {
        info!("uniform_3f(location: {}, v0: {}, v1: {}, v2: {})", location, v0, v1, v2);
        self.gl.uniform_3f(location, v0, v1, v2)
    }
    fn uniform_3fv(&self, location: i32, values: &[f32]) {
        info!("uniform_3fv(location: {}, values: ({}))", location, values.len());
        self.gl.uniform_3fv(location, values)
    }
    fn uniform_3i(&self, location: i32, v0: i32, v1: i32, v2: i32) {
        info!("uniform_3i(location: {}, v0: {}, v1: {}, v2: {})", location, v0, v1, v2);
        self.gl.uniform_3i(location, v0, v1, v2)
    }
    fn uniform_3iv(&self, location: i32, values: &[i32]) {
        info!("uniform_3iv(location: {}, values: ({}))", location, values.len());
        self.gl.uniform_3iv(location, values)
    }
    fn uniform_3ui(&self, location: i32, v0: u32, v1: u32, v2: u32) {
        info!("uniform_3ui(location: {}, v0: {}, v1: {}, v2: {})", location, v0, v1, v2);
        self.gl.uniform_3ui(location, v0, v1, v2)
    }
    fn uniform_4f(&self, location: i32, x: f32, y: f32, z: f32, w: f32) {
        info!("uniform_4f(location: {}, x: {}, y: {}, z: {}, w: {})", location, x, y, z, w);
        self.gl.uniform_4f(location, x, y, z, w)
    }
    fn uniform_4i(&self, location: i32, x: i32, y: i32, z: i32, w: i32) {
        info!("uniform_4i(location: {}, x: {}, y: {}, z: {}, w: {})", location, x, y, z, w);
        self.gl.uniform_4i(location, x, y, z, w)
    }
    fn uniform_4iv(&self, location: i32, values: &[i32]) {
        info!("uniform_4iv(location: {}, values: ({}))", location, values.len());
        self.gl.uniform_4iv(location, values)
    }
    fn uniform_4ui(&self, location: i32, x: u32, y: u32, z: u32, w: u32) {
        info!("uniform_4ui(location: {}, x: {}, y: {}, z: {}, w: {})", location, x, y, z, w);
        self.gl.uniform_4ui(location, x, y, z, w)
    }
    fn uniform_4fv(&self, location: i32, values: &[f32]) {
        info!("uniform_4fv(location: {}, values: ({}))", location, values.len());
        self.gl.uniform_4fv(location, values)
    }
    fn uniform_matrix_2fv(&self, location: i32, transpose: bool, value: &[f32]) {
        info!("uniform_matrix_2fv(location: {}, transpose: {}, value: ({}))", location, transpose, value.len());
        self.gl.uniform_matrix_2fv(location, transpose, value)
    }
    fn uniform_matrix_3fv(&self, location: i32, transpose: bool, value: &[f32]) {
        info!("uniform_matrix_3fv(location: {}, transpose: {}, value: ({}))", location, transpose, value.len());
        self.gl.uniform_matrix_3fv(location, transpose, value)
    }
    fn uniform_matrix_4fv(&self, location: i32, transpose: bool, value: &[f32]) {
        info!("uniform_matrix_4fv(location: {}, transpose: {}, value: ({}))", location, transpose, value.len());
        self.gl.uniform_matrix_4fv(location, transpose, value)
    }
    fn depth_mask(&self, flag: bool) {
        info!("depth_mask(flag: {})", flag);
        self.gl.depth_mask(flag)
    }
    fn depth_range(&self, near: f64, far: f64) {
        info!("depth_range(near: {}, far: {})", near, far);
        self.gl.depth_range(near, far)
    }
    fn get_active_attrib(&self, program: u32, index: u32) -> (i32, u32, String) {
        info!("get_active_attrib(program: {}, index: {})", program, index);
        self.gl.get_active_attrib(program, index)
    }
    fn get_active_uniform(&self, program: u32, index: u32) -> (i32, u32, String) {
        info!("get_active_uniform(program: {}, index: {})", program, index);
        self.gl.get_active_uniform(program, index)
    }
    fn get_active_uniforms_iv(
        &self, 
        program: u32, 
        indices: Vec<u32>, 
        pname: u32
    ) -> Vec<i32> {
        info!("get_active_uniforms_iv(program: {}, indices: ({}), pname: {})", program, indices.len(), pname);
        self.gl.get_active_uniforms_iv(program, indices, pname)
    }
    fn get_active_uniform_block_i(
        &self, 
        program: u32, 
        index: u32, 
        pname: u32
    ) -> i32 {
        info!("get_active_uniform_block_i(program: {}, index: {}, pname: {})", program, index, pname);
        self.gl.get_active_uniform_block_i(program, index, pname)
    }
    fn get_active_uniform_block_iv(
        &self, 
        program: u32, 
        index: u32, 
        pname: u32
    ) -> Vec<i32> {
        info!("get_active_uniform_block_iv(program: {}, index: {}, pname: {})", program, index, pname);
        self.gl.get_active_uniform_block_iv(program, index, pname)
    }
    fn get_active_uniform_block_name(&self, program: u32, index: u32) -> String {
        info!("get_active_uniform_block_name(program: {}, index: {})", program, index);
        self.gl.get_active_uniform_block_name(program, index)
    }
    fn get_attrib_location(&self, program: u32, name: &str) -> i32 {
        info!("get_attrib_location(program: {}, name: {})", program, name);
        self.gl.get_attrib_location(program, name)
    }
    fn get_frag_data_location(&self, program: u32, name: &str) -> i32 {
        info!("get_frag_data_location(program: {}, name: {})", program, name);
        self.gl.get_frag_data_location(program, name)
    }
    fn get_uniform_location(&self, program: u32, name: &str) -> i32 {
        info!("get_uniform_location(program: {}, name: {})", program, name);
        self.gl.get_uniform_location(program, name)
    }
    fn get_program_info_log(&self, program: u32) -> String {
        info!("get_program_info_log(program: {})", program);
        self.gl.get_program_info_log(program)
    }
    fn get_program_iv(&self, program: u32, pname: u32) -> i32 {
        info!("get_program_iv(program: {}, pname: {})", program, pname);
        self.gl.get_program_iv(program, pname)
    }
    fn get_program_binary(&self, program: u32) -> (Vec<u8>, u32) {
        info!("get_program_binary(program: {})", program);
        self.gl.get_program_binary(program)
    }
    fn program_binary(&self, program: u32, format: u32, binary: &[u8]) {
        info!("program_binary(program: {}, format: {}, binary: ({}))", program, format, binary.len());
        self.gl.program_binary(program, format, binary)
    }
    fn program_parameter_i(&self, program: u32, pname: u32, value: i32) {
        info!("program_parameter_i(program: {}, pname: {}, value: {})", program, pname, value);
        self.gl.program_parameter_i(program, pname, value)
    }
    fn get_vertex_attrib_iv(&self, index: u32, pname: u32) -> i32 {
        info!("get_vertex_attrib_iv(index: {}, pname: {})", index, pname);
        self.gl.get_vertex_attrib_iv(index, pname)
    }
    fn get_vertex_attrib_fv(&self, index: u32, pname: u32) -> Vec<f32> {
        info!("get_vertex_attrib_fv(index: {}, pname: {})", index, pname);
        self.gl.get_vertex_attrib_fv(index, pname)
    }
    fn get_vertex_attrib_pointer_v(&self, index: u32, pname: u32) -> isize {
        info!("get_vertex_attrib_pointer_v(index: {}, pname: {})", index, pname);
        self.gl.get_vertex_attrib_pointer_v(index, pname)
    }
    fn get_buffer_parameter_iv(&self, target: u32, pname: u32) -> i32 {
        info!("get_buffer_parameter_iv(target: {}, pname: {})", target, pname);
        self.gl.get_buffer_parameter_iv(target, pname)
    }
    fn get_shader_info_log(&self, shader: u32) -> String {
        info!("get_shader_info_log(shader: {})", shader);
        self.gl.get_shader_info_log(shader)
    }
    fn get_string(&self, which: u32) -> String {
        info!("get_string(which: {})", which);
        self.gl.get_string(which)
    }
    fn get_string_i(&self, which: u32, index: u32) -> String {
        info!("get_string_i(which: {}, index: {})", which, index);
        self.gl.get_string_i(which, index)
    }
    fn get_shader_iv(&self, shader: u32, pname: u32) -> i32 {
        info!("get_shader_iv(shader: {}, pname: {})", shader, pname);
        self.gl.get_shader_iv(shader, pname)
    }
    fn get_shader_precision_format(
        &self, 
        shader_type: u32, 
        precision_type: u32
    ) -> (i32, i32, i32) {
        info!("get_shader_precision_format(shader_type: {}, precision_type: {})", shader_type, precision_type);
        self.gl.get_shader_precision_format(shader_type, precision_type)
    }
    fn compile_shader(&self, shader: u32) {
        info!("compile_shader(shader: {})", shader);
        self.gl.compile_shader(shader)
    }
    fn create_program(&self) -> u32 {
        let program = self.gl.create_program();
        info!("create_program() -> {}", program);
        program
    }
    fn delete_program(&self, program: u32) {
        info!("delete_program(program: {})", program);
        self.gl.delete_program(program)
    }
    fn create_shader(&self, shader_type: u32) -> u32 {
        let r = self.gl.create_shader(shader_type);
        info!("create_shader(shader_type: {}) -> {}", shader_type, r);
        r
    }
    fn delete_shader(&self, shader: u32) {
        info!("delete_shader(shader: {})", shader);
        self.gl.delete_shader(shader)
    }
    fn detach_shader(&self, program: u32, shader: u32) {
        info!("detach_shader(program: {}, shader: {})", program, shader);
        self.gl.detach_shader(program, shader)
    }
    fn link_program(&self, program: u32) {
        info!("link_program(program: {})", program);
        self.gl.link_program(program)
    }
    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        info!("clear_color(r: {}, g: {}, b: {}, a: {})", r, g, b, a);
        self.gl.clear_color(r, g, b, a)
    }
    fn clear(&self, buffer_mask: u32) {
        info!("clear(buffer_mask: {})", buffer_mask);
        self.gl.clear(buffer_mask)
    }
    fn clear_depth(&self, depth: f64) {
        info!("clear_depth(depth: {})", depth);
        self.gl.clear_depth(depth)
    }
    fn clear_stencil(&self, s: i32) {
        info!("clear_stencil(s: {})", s);
        self.gl.clear_stencil(s)
    }
    fn flush(&self) {
        info!("flush");
        self.gl.flush()
    }
    fn finish(&self) {
        info!("finish");
        self.gl.finish()
    }
    fn get_error(&self) -> u32 {
        info!("get_error");
        self.gl.get_error()
    }
    fn stencil_mask(&self, mask: u32) {
        info!("stencil_mask(mask: {})", mask);
        self.gl.stencil_mask(mask)
    }
    fn stencil_mask_separate(&self, face: u32, mask: u32) {
        info!("stencil_mask_separate(face: {}, mask: {})", face, mask);
        self.gl.stencil_mask_separate(face, mask)
    }
    fn stencil_func(&self, func: u32, ref_: i32, mask: u32) {
        info!("stencil_func(func: {}, ref_: {}, mask: {})", func, ref_, mask);
        self.gl.stencil_func(func, ref_, mask)
    }
    fn stencil_func_separate(&self, face: u32, func: u32, ref_: i32, mask: u32) {
        info!("stencil_func_separate(face: {}, func: {}, ref_: {}, mask: {})", face, func, ref_, mask);
        self.gl.stencil_func_separate(face, func, ref_, mask)
    }
    fn stencil_op(&self, sfail: u32, dpfail: u32, dppass: u32) {
        info!("stencil_op(sfail: {}, dpfail: {}, dppass: {})", sfail, dpfail, dppass);
        self.gl.stencil_op(sfail, dpfail, dppass)
    }
    fn stencil_op_separate(
        &self, 
        face: u32, 
        sfail: u32, 
        dpfail: u32, 
        dppass: u32
    ) {
        info!("stencil_op_separate(face: {}, sfail: {}, dpfail: {}, dppass: {})", face, sfail, dpfail, dppass);
        self.gl.stencil_op_separate(face, sfail, dpfail, dppass)
    }
    fn egl_image_target_texture2d_oes(&self, target: u32, image: *const c_void) {
        info!("egl_image_target_texture2d_oes(target: {}, image: {:?})", target, image);
        self.gl.egl_image_target_texture2d_oes(target, image)
    }
    fn generate_mipmap(&self, target: u32) {
        info!("generate_mipmap(target: {})", target);
        self.gl.generate_mipmap(target)
    }
    fn insert_event_marker_ext(&self, message: &str) {
        info!("insert_event_marker_ext(message: {})", message);
        self.gl.insert_event_marker_ext(message)
    }
    fn push_group_marker_ext(&self, message: &str) {
        info!("push_group_marker_ext(message: {})", message);
        self.gl.push_group_marker_ext(message)
    }
    fn pop_group_marker_ext(&self) {
        info!("pop_group_marker_ext");
        self.gl.pop_group_marker_ext()
    }
    fn fence_sync(&self, condition: u32, flags: u32) -> *const __GLsync {
        info!("fence_sync(condition: {}, flags: {})", condition, flags);
        self.gl.fence_sync(condition, flags)
    }
    fn client_wait_sync(&self, sync: *const __GLsync, flags: u32, timeout: u64) {
        info!("client_wait_sync(sync: {:?}, flags: {}, timeout: {})", sync, flags, timeout);
        self.gl.client_wait_sync(sync, flags, timeout)
    }
    fn wait_sync(&self, sync: *const __GLsync, flags: u32, timeout: u64) {
        info!("wait_sync(sync: {:?}, flags: {}, timeout: {})", sync, flags, timeout);
        self.gl.wait_sync(sync, flags, timeout)
    }
    fn delete_sync(&self, sync: *const __GLsync) {
        info!("delete_sync(sync: {:?})", sync);
        self.gl.delete_sync(sync)
    }
}