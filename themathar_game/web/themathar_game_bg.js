/**
 * @param {number} lobby_id
 * @param {number} player_id
 */
export function receive_identity(lobby_id, player_id) {
    wasm.receive_identity(lobby_id, player_id);
}

/**
 * @param {number} active_player_id
 * @param {number} turn_number
 */
export function receive_state(active_player_id, turn_number) {
    wasm.receive_state(active_player_id, turn_number);
}

export function run_game() {
    wasm.run_game();
}
export function __wbg_Window_70c6d673c246c927(arg0) {
    const ret = getObject(arg0).Window;
    return addHeapObject(ret);
}
export function __wbg_Window_d1bf622f71ff0629(arg0) {
    const ret = getObject(arg0).Window;
    return addHeapObject(ret);
}
export function __wbg_WorkerGlobalScope_d1c929ee694c77f5(arg0) {
    const ret = getObject(arg0).WorkerGlobalScope;
    return addHeapObject(ret);
}
export function __wbg___wbindgen_boolean_get_bbbb1c18aa2f5e25(arg0) {
    const v = getObject(arg0);
    const ret = typeof(v) === 'boolean' ? v : undefined;
    return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
}
export function __wbg___wbindgen_debug_string_0bc8482c6e3508ae(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_is_function_0095a73b8b156f76(arg0) {
    const ret = typeof(getObject(arg0)) === 'function';
    return ret;
}
export function __wbg___wbindgen_is_null_ac34f5003991759a(arg0) {
    const ret = getObject(arg0) === null;
    return ret;
}
export function __wbg___wbindgen_is_undefined_9e4d92534c42d778(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
}
export function __wbg___wbindgen_number_get_8ff4255516ccad3e(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
}
export function __wbg___wbindgen_string_get_72fb696202c56729(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg___wbindgen_throw_be289d5034ed271b(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg__wbg_cb_unref_d9b87ff7982e3b21(arg0) {
    getObject(arg0)._wbg_cb_unref();
}
export function __wbg_abort_2f0584e03e8e3950(arg0) {
    getObject(arg0).abort();
}
export function __wbg_activeElement_1554b6917654f8d6(arg0) {
    const ret = getObject(arg0).activeElement;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_activeTexture_6f9a710514686c24(arg0, arg1) {
    getObject(arg0).activeTexture(arg1 >>> 0);
}
export function __wbg_activeTexture_7e39cb8fdf4b6d5a(arg0, arg1) {
    getObject(arg0).activeTexture(arg1 >>> 0);
}
export function __wbg_addEventListener_3acb0aad4483804c() { return handleError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
}, arguments); }
export function __wbg_addListener_03e8162d7e03c823() { return handleError(function (arg0, arg1) {
    getObject(arg0).addListener(getObject(arg1));
}, arguments); }
export function __wbg_altKey_73c1173ba53073d5(arg0) {
    const ret = getObject(arg0).altKey;
    return ret;
}
export function __wbg_altKey_8155c319c215e3aa(arg0) {
    const ret = getObject(arg0).altKey;
    return ret;
}
export function __wbg_animate_6ec571f163cf6f8d(arg0, arg1, arg2) {
    const ret = getObject(arg0).animate(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}
export function __wbg_appendChild_dea38765a26d346d() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_arrayBuffer_bb54076166006c39() { return handleError(function (arg0) {
    const ret = getObject(arg0).arrayBuffer();
    return addHeapObject(ret);
}, arguments); }
export function __wbg_attachShader_32114efcf2744eb6(arg0, arg1, arg2) {
    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
}
export function __wbg_attachShader_b36058e5c9eeaf54(arg0, arg1, arg2) {
    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
}
export function __wbg_axes_6c53f544c314fe19(arg0) {
    const ret = getObject(arg0).axes;
    return addHeapObject(ret);
}
export function __wbg_beginQuery_0fdf154e1da0e73d(arg0, arg1, arg2) {
    getObject(arg0).beginQuery(arg1 >>> 0, getObject(arg2));
}
export function __wbg_bindAttribLocation_5cfc7fa688df5051(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).bindAttribLocation(getObject(arg1), arg2 >>> 0, getStringFromWasm0(arg3, arg4));
}
export function __wbg_bindAttribLocation_ce78bfb13019dbe6(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).bindAttribLocation(getObject(arg1), arg2 >>> 0, getStringFromWasm0(arg3, arg4));
}
export function __wbg_bindBufferRange_009d206fe9e4151e(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).bindBufferRange(arg1 >>> 0, arg2 >>> 0, getObject(arg3), arg4, arg5);
}
export function __wbg_bindBuffer_69a7a0b8f3f9b9cf(arg0, arg1, arg2) {
    getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
}
export function __wbg_bindBuffer_c9068e8712a034f5(arg0, arg1, arg2) {
    getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
}
export function __wbg_bindFramebuffer_031c73ba501cb8f6(arg0, arg1, arg2) {
    getObject(arg0).bindFramebuffer(arg1 >>> 0, getObject(arg2));
}
export function __wbg_bindFramebuffer_7815ca611abb057f(arg0, arg1, arg2) {
    getObject(arg0).bindFramebuffer(arg1 >>> 0, getObject(arg2));
}
export function __wbg_bindRenderbuffer_8a2aa4e3d1fb5443(arg0, arg1, arg2) {
    getObject(arg0).bindRenderbuffer(arg1 >>> 0, getObject(arg2));
}
export function __wbg_bindRenderbuffer_db37c1bac9ed4da0(arg0, arg1, arg2) {
    getObject(arg0).bindRenderbuffer(arg1 >>> 0, getObject(arg2));
}
export function __wbg_bindSampler_96f0e90e7bc31da9(arg0, arg1, arg2) {
    getObject(arg0).bindSampler(arg1 >>> 0, getObject(arg2));
}
export function __wbg_bindTexture_b2b7b1726a83f93e(arg0, arg1, arg2) {
    getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
}
export function __wbg_bindTexture_ec13ddcb9dc8e032(arg0, arg1, arg2) {
    getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
}
export function __wbg_bindVertexArrayOES_c2610602f7485b3f(arg0, arg1) {
    getObject(arg0).bindVertexArrayOES(getObject(arg1));
}
export function __wbg_bindVertexArray_78220d1edb1d2382(arg0, arg1) {
    getObject(arg0).bindVertexArray(getObject(arg1));
}
export function __wbg_blendColor_1d50ac87d9a2794b(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).blendColor(arg1, arg2, arg3, arg4);
}
export function __wbg_blendColor_e799d452ab2a5788(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).blendColor(arg1, arg2, arg3, arg4);
}
export function __wbg_blendEquationSeparate_1b12c43928cc7bc1(arg0, arg1, arg2) {
    getObject(arg0).blendEquationSeparate(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_blendEquationSeparate_a8094fbec94cf80e(arg0, arg1, arg2) {
    getObject(arg0).blendEquationSeparate(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_blendEquation_82202f34c4c00e50(arg0, arg1) {
    getObject(arg0).blendEquation(arg1 >>> 0);
}
export function __wbg_blendEquation_e9b99928ed1494ad(arg0, arg1) {
    getObject(arg0).blendEquation(arg1 >>> 0);
}
export function __wbg_blendFuncSeparate_95465944f788a092(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).blendFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
}
export function __wbg_blendFuncSeparate_f366c170c5097fbe(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).blendFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
}
export function __wbg_blendFunc_2ef59299d10c662d(arg0, arg1, arg2) {
    getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_blendFunc_446658e7231ab9c8(arg0, arg1, arg2) {
    getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_blitFramebuffer_d730a23ab4db248e(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    getObject(arg0).blitFramebuffer(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0);
}
export function __wbg_blockSize_ef9a626745d7dfac(arg0) {
    const ret = getObject(arg0).blockSize;
    return ret;
}
export function __wbg_body_f67922363a220026(arg0) {
    const ret = getObject(arg0).body;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_brand_9562792cbb4735c3(arg0, arg1) {
    const ret = getObject(arg1).brand;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_brands_a1e7a2bce052128f(arg0) {
    const ret = getObject(arg0).brands;
    return addHeapObject(ret);
}
export function __wbg_bufferData_1be8450fab534758(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
}
export function __wbg_bufferData_32d26eba0c74a53c(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
}
export function __wbg_bufferData_52235e85894af988(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
}
export function __wbg_bufferData_98f6c413a8f0f139(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
}
export function __wbg_bufferSubData_33eebcc173094f6a(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferSubData(arg1 >>> 0, arg2, getObject(arg3));
}
export function __wbg_bufferSubData_3e902f031adf13fd(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferSubData(arg1 >>> 0, arg2, getObject(arg3));
}
export function __wbg_button_d86841d0a03adc44(arg0) {
    const ret = getObject(arg0).button;
    return ret;
}
export function __wbg_buttons_172c77c2ce62b90c(arg0) {
    const ret = getObject(arg0).buttons;
    return addHeapObject(ret);
}
export function __wbg_buttons_a158a0cad3175f24(arg0) {
    const ret = getObject(arg0).buttons;
    return ret;
}
export function __wbg_call_389efe28435a9388() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_cancelAnimationFrame_cd35895d78cf4510() { return handleError(function (arg0, arg1) {
    getObject(arg0).cancelAnimationFrame(arg1);
}, arguments); }
export function __wbg_cancelIdleCallback_fdfaaf4ca585e729(arg0, arg1) {
    getObject(arg0).cancelIdleCallback(arg1 >>> 0);
}
export function __wbg_cancel_09c394f0894744eb(arg0) {
    getObject(arg0).cancel();
}
export function __wbg_catch_c1f8c7623b458214(arg0, arg1) {
    const ret = getObject(arg0).catch(getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_clearBufferfv_ac87d92e2f45d80c(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearBufferfv(arg1 >>> 0, arg2, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_clearBufferiv_69ff24bb52ec4c88(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearBufferiv(arg1 >>> 0, arg2, getArrayI32FromWasm0(arg3, arg4));
}
export function __wbg_clearBufferuiv_8ad59a8219aafaca(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearBufferuiv(arg1 >>> 0, arg2, getArrayU32FromWasm0(arg3, arg4));
}
export function __wbg_clearDepth_2b109f644a783a53(arg0, arg1) {
    getObject(arg0).clearDepth(arg1);
}
export function __wbg_clearDepth_670099db422a4f91(arg0, arg1) {
    getObject(arg0).clearDepth(arg1);
}
export function __wbg_clearStencil_5d243d0dff03c315(arg0, arg1) {
    getObject(arg0).clearStencil(arg1);
}
export function __wbg_clearStencil_aa65955bb39d8c18(arg0, arg1) {
    getObject(arg0).clearStencil(arg1);
}
export function __wbg_clearTimeout_df03cf00269bc442(arg0, arg1) {
    getObject(arg0).clearTimeout(arg1);
}
export function __wbg_clear_4d801d0d054c3579(arg0, arg1) {
    getObject(arg0).clear(arg1 >>> 0);
}
export function __wbg_clear_7187030f892c5ca0(arg0, arg1) {
    getObject(arg0).clear(arg1 >>> 0);
}
export function __wbg_clientWaitSync_21865feaeb76a9a5(arg0, arg1, arg2, arg3) {
    const ret = getObject(arg0).clientWaitSync(getObject(arg1), arg2 >>> 0, arg3 >>> 0);
    return ret;
}
export function __wbg_close_987a203f749ce4ab() { return handleError(function (arg0) {
    const ret = getObject(arg0).close();
    return addHeapObject(ret);
}, arguments); }
export function __wbg_close_fad2f0ee451926ed(arg0) {
    getObject(arg0).close();
}
export function __wbg_code_dee0dae4730408e1(arg0, arg1) {
    const ret = getObject(arg1).code;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_colorMask_177d9762658e5e28(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).colorMask(arg1 !== 0, arg2 !== 0, arg3 !== 0, arg4 !== 0);
}
export function __wbg_colorMask_7a8dbc86e7376a9b(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).colorMask(arg1 !== 0, arg2 !== 0, arg3 !== 0, arg4 !== 0);
}
export function __wbg_compileShader_63b824e86bb00b8f(arg0, arg1) {
    getObject(arg0).compileShader(getObject(arg1));
}
export function __wbg_compileShader_94718a93495d565d(arg0, arg1) {
    getObject(arg0).compileShader(getObject(arg1));
}
export function __wbg_compressedTexSubImage2D_215bb115facd5e48(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
    getObject(arg0).compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, getObject(arg8));
}
export function __wbg_compressedTexSubImage2D_684350eb62830032(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
    getObject(arg0).compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, getObject(arg8));
}
export function __wbg_compressedTexSubImage2D_d8fbae93bb8c4cc9(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).compressedTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8, arg9);
}
export function __wbg_compressedTexSubImage3D_16afa3a47bf1d979(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    getObject(arg0).compressedTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, getObject(arg10));
}
export function __wbg_compressedTexSubImage3D_778008a6293f15ab(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    getObject(arg0).compressedTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10, arg11);
}
export function __wbg_connect_aba749effbe588ea() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).connect(getObject(arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_connected_6d06e0c95c0d644a(arg0) {
    const ret = getObject(arg0).connected;
    return ret;
}
export function __wbg_contains_1056459c33f961e8(arg0, arg1) {
    const ret = getObject(arg0).contains(getObject(arg1));
    return ret;
}
export function __wbg_contentRect_79b98e4d4f4728a4(arg0) {
    const ret = getObject(arg0).contentRect;
    return addHeapObject(ret);
}
export function __wbg_copyBufferSubData_a4f9815861ff0ae9(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).copyBufferSubData(arg1 >>> 0, arg2 >>> 0, arg3, arg4, arg5);
}
export function __wbg_copyTexSubImage2D_417a65926e3d2490(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
    getObject(arg0).copyTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
}
export function __wbg_copyTexSubImage2D_91ebcd9cd1908265(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
    getObject(arg0).copyTexSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
}
export function __wbg_copyTexSubImage3D_f62ef4c4eeb9a7dc(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).copyTexSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
}
export function __wbg_copyToChannel_4bfc91363fcbe558() { return handleError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).copyToChannel(getArrayF32FromWasm0(arg1, arg2), arg3);
}, arguments); }
export function __wbg_createBufferSource_1254cd048e6c0593() { return handleError(function (arg0) {
    const ret = getObject(arg0).createBufferSource();
    return addHeapObject(ret);
}, arguments); }
export function __wbg_createBuffer_26534c05e01b8559(arg0) {
    const ret = getObject(arg0).createBuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createBuffer_36fe5fb1d9f77b9d() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = getObject(arg0).createBuffer(arg1 >>> 0, arg2 >>> 0, arg3);
    return addHeapObject(ret);
}, arguments); }
export function __wbg_createBuffer_c4ec897aacc1b91c(arg0) {
    const ret = getObject(arg0).createBuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createElement_49f60fdcaae809c8() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_createFramebuffer_41512c38358a41c4(arg0) {
    const ret = getObject(arg0).createFramebuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createFramebuffer_b88ffa8e0fd262c4(arg0) {
    const ret = getObject(arg0).createFramebuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createImageBitmap_027110a9da500302() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).createImageBitmap(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_createObjectURL_918185db6a10a0c8() { return handleError(function (arg0, arg1) {
    const ret = URL.createObjectURL(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments); }
export function __wbg_createProgram_98aaa91f7c81c5e2(arg0) {
    const ret = getObject(arg0).createProgram();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createProgram_9b7710a1f2701c2c(arg0) {
    const ret = getObject(arg0).createProgram();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createQuery_7988050efd7e4c48(arg0) {
    const ret = getObject(arg0).createQuery();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createRenderbuffer_1e567f2f4d461710(arg0) {
    const ret = getObject(arg0).createRenderbuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createRenderbuffer_a601226a6a680dbe(arg0) {
    const ret = getObject(arg0).createRenderbuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createSampler_da6bb96c9ffaaa27(arg0) {
    const ret = getObject(arg0).createSampler();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createShader_e3ac08ed8c5b14b2(arg0, arg1) {
    const ret = getObject(arg0).createShader(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createShader_f2b928ca9a426b14(arg0, arg1) {
    const ret = getObject(arg0).createShader(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createTexture_16d2c8a3d7d4a75a(arg0) {
    const ret = getObject(arg0).createTexture();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createTexture_f9451a82c7527ce2(arg0) {
    const ret = getObject(arg0).createTexture();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createVertexArrayOES_bd76ceee6ab9b95e(arg0) {
    const ret = getObject(arg0).createVertexArrayOES();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_createVertexArray_ad5294951ae57497(arg0) {
    const ret = getObject(arg0).createVertexArray();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_ctrlKey_09a1b54d77dea92b(arg0) {
    const ret = getObject(arg0).ctrlKey;
    return ret;
}
export function __wbg_ctrlKey_96ff94f8b18636a3(arg0) {
    const ret = getObject(arg0).ctrlKey;
    return ret;
}
export function __wbg_cullFace_39500f654c67a205(arg0, arg1) {
    getObject(arg0).cullFace(arg1 >>> 0);
}
export function __wbg_cullFace_e7e711a14d2c3f48(arg0, arg1) {
    getObject(arg0).cullFace(arg1 >>> 0);
}
export function __wbg_currentTime_6c7288048dba47fa(arg0) {
    const ret = getObject(arg0).currentTime;
    return ret;
}
export function __wbg_decode_322508cb812e789d(arg0) {
    const ret = getObject(arg0).decode();
    return addHeapObject(ret);
}
export function __wbg_deleteBuffer_22fcc93912cbf659(arg0, arg1) {
    getObject(arg0).deleteBuffer(getObject(arg1));
}
export function __wbg_deleteBuffer_ab099883c168644d(arg0, arg1) {
    getObject(arg0).deleteBuffer(getObject(arg1));
}
export function __wbg_deleteFramebuffer_8de1ca41ac87cfd9(arg0, arg1) {
    getObject(arg0).deleteFramebuffer(getObject(arg1));
}
export function __wbg_deleteFramebuffer_9738f3bb85c1ab35(arg0, arg1) {
    getObject(arg0).deleteFramebuffer(getObject(arg1));
}
export function __wbg_deleteProgram_9298fb3e3c1d3a78(arg0, arg1) {
    getObject(arg0).deleteProgram(getObject(arg1));
}
export function __wbg_deleteProgram_f354e79b8cae8076(arg0, arg1) {
    getObject(arg0).deleteProgram(getObject(arg1));
}
export function __wbg_deleteQuery_ea8bf1954febd774(arg0, arg1) {
    getObject(arg0).deleteQuery(getObject(arg1));
}
export function __wbg_deleteRenderbuffer_096edada57729468(arg0, arg1) {
    getObject(arg0).deleteRenderbuffer(getObject(arg1));
}
export function __wbg_deleteRenderbuffer_0f565f0727b341fc(arg0, arg1) {
    getObject(arg0).deleteRenderbuffer(getObject(arg1));
}
export function __wbg_deleteSampler_c6b68c4071841afa(arg0, arg1) {
    getObject(arg0).deleteSampler(getObject(arg1));
}
export function __wbg_deleteShader_aaf3b520a64d5d9d(arg0, arg1) {
    getObject(arg0).deleteShader(getObject(arg1));
}
export function __wbg_deleteShader_ff70ca962883e241(arg0, arg1) {
    getObject(arg0).deleteShader(getObject(arg1));
}
export function __wbg_deleteSync_c8e4a9c735f71d18(arg0, arg1) {
    getObject(arg0).deleteSync(getObject(arg1));
}
export function __wbg_deleteTexture_2be78224e5584a8b(arg0, arg1) {
    getObject(arg0).deleteTexture(getObject(arg1));
}
export function __wbg_deleteTexture_9d411c0e60ffa324(arg0, arg1) {
    getObject(arg0).deleteTexture(getObject(arg1));
}
export function __wbg_deleteVertexArrayOES_197df47ef9684195(arg0, arg1) {
    getObject(arg0).deleteVertexArrayOES(getObject(arg1));
}
export function __wbg_deleteVertexArray_7bc7f92769862f93(arg0, arg1) {
    getObject(arg0).deleteVertexArray(getObject(arg1));
}
export function __wbg_deltaMode_a1d1df711e44cefc(arg0) {
    const ret = getObject(arg0).deltaMode;
    return ret;
}
export function __wbg_deltaX_f0ca9116db5f7bc1(arg0) {
    const ret = getObject(arg0).deltaX;
    return ret;
}
export function __wbg_deltaY_eb94120160ac821c(arg0) {
    const ret = getObject(arg0).deltaY;
    return ret;
}
export function __wbg_depthFunc_eb3aa05361dd2eaa(arg0, arg1) {
    getObject(arg0).depthFunc(arg1 >>> 0);
}
export function __wbg_depthFunc_f670d4cbb9cd0913(arg0, arg1) {
    getObject(arg0).depthFunc(arg1 >>> 0);
}
export function __wbg_depthMask_103091329ca1a750(arg0, arg1) {
    getObject(arg0).depthMask(arg1 !== 0);
}
export function __wbg_depthMask_75a36d0065471a4b(arg0, arg1) {
    getObject(arg0).depthMask(arg1 !== 0);
}
export function __wbg_depthRange_337bf254e67639bb(arg0, arg1, arg2) {
    getObject(arg0).depthRange(arg1, arg2);
}
export function __wbg_depthRange_5579d448b9d7de57(arg0, arg1, arg2) {
    getObject(arg0).depthRange(arg1, arg2);
}
export function __wbg_destination_a97dbc327ce97191(arg0) {
    const ret = getObject(arg0).destination;
    return addHeapObject(ret);
}
export function __wbg_devicePixelContentBoxSize_8f39437eab7f03ea(arg0) {
    const ret = getObject(arg0).devicePixelContentBoxSize;
    return addHeapObject(ret);
}
export function __wbg_devicePixelRatio_5c458affc89fc209(arg0) {
    const ret = getObject(arg0).devicePixelRatio;
    return ret;
}
export function __wbg_disableVertexAttribArray_24a020060006b10f(arg0, arg1) {
    getObject(arg0).disableVertexAttribArray(arg1 >>> 0);
}
export function __wbg_disableVertexAttribArray_4bac633c27bae599(arg0, arg1) {
    getObject(arg0).disableVertexAttribArray(arg1 >>> 0);
}
export function __wbg_disable_7fe6fb3e97717f88(arg0, arg1) {
    getObject(arg0).disable(arg1 >>> 0);
}
export function __wbg_disable_bd37bdcca1764aea(arg0, arg1) {
    getObject(arg0).disable(arg1 >>> 0);
}
export function __wbg_disconnect_0a2d26237dfc1e9e(arg0) {
    getObject(arg0).disconnect();
}
export function __wbg_disconnect_5202f399852258c0(arg0) {
    getObject(arg0).disconnect();
}
export function __wbg_document_ee35a3d3ae34ef6c(arg0) {
    const ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_drawArraysInstancedANGLE_9e4cc507eae8b24d(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).drawArraysInstancedANGLE(arg1 >>> 0, arg2, arg3, arg4);
}
export function __wbg_drawArraysInstanced_ec30adc616ec58d5(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).drawArraysInstanced(arg1 >>> 0, arg2, arg3, arg4);
}
export function __wbg_drawArrays_075228181299b824(arg0, arg1, arg2, arg3) {
    getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
}
export function __wbg_drawArrays_2be89c369a29f30b(arg0, arg1, arg2, arg3) {
    getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
}
export function __wbg_drawBuffersWEBGL_447bc0a21f8ef22d(arg0, arg1) {
    getObject(arg0).drawBuffersWEBGL(getObject(arg1));
}
export function __wbg_drawBuffers_5eccfaacc6560299(arg0, arg1) {
    getObject(arg0).drawBuffers(getObject(arg1));
}
export function __wbg_drawElementsInstancedANGLE_6f9da0b845ac6c4e(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).drawElementsInstancedANGLE(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
}
export function __wbg_drawElementsInstanced_d41fc920ae24717c(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).drawElementsInstanced(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
}
export function __wbg_enableVertexAttribArray_475e06c31777296d(arg0, arg1) {
    getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
}
export function __wbg_enableVertexAttribArray_aa6e40408261eeb9(arg0, arg1) {
    getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
}
export function __wbg_enable_d1ac04dfdd2fb3ae(arg0, arg1) {
    getObject(arg0).enable(arg1 >>> 0);
}
export function __wbg_enable_fee40f19b7053ea3(arg0, arg1) {
    getObject(arg0).enable(arg1 >>> 0);
}
export function __wbg_endQuery_54f0627d4c931318(arg0, arg1) {
    getObject(arg0).endQuery(arg1 >>> 0);
}
export function __wbg_error_7534b8e9a36f1ab4(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_export4(deferred0_0, deferred0_1, 1);
    }
}
export function __wbg_error_f852e41c69b0bd84(arg0, arg1) {
    console.error(getObject(arg0), getObject(arg1));
}
export function __wbg_eval_3f0b9f0cbaf45a34() { return handleError(function (arg0, arg1) {
    const ret = eval(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_exec_48e0e0ad953102ac(arg0, arg1, arg2) {
    const ret = getObject(arg0).exec(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_exitFullscreen_a15f439a0e27b307(arg0) {
    getObject(arg0).exitFullscreen();
}
export function __wbg_exitPointerLock_faff71a5e2d467ea(arg0) {
    getObject(arg0).exitPointerLock();
}
export function __wbg_fenceSync_c52a4e24eabfa0d3(arg0, arg1, arg2) {
    const ret = getObject(arg0).fenceSync(arg1 >>> 0, arg2 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_fetch_4f06ca81d87798ba(arg0, arg1, arg2) {
    const ret = getObject(arg0).fetch(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}
export function __wbg_fetch_d1488f40cef1e210(arg0, arg1, arg2) {
    const ret = getObject(arg0).fetch(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}
export function __wbg_flush_7777597fd43065db(arg0) {
    getObject(arg0).flush();
}
export function __wbg_flush_e322496f5412e567(arg0) {
    getObject(arg0).flush();
}
export function __wbg_focus_128ff465f65677cc() { return handleError(function (arg0) {
    getObject(arg0).focus();
}, arguments); }
export function __wbg_framebufferRenderbuffer_850811ed6e26475e(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4));
}
export function __wbg_framebufferRenderbuffer_cd9d55a68a2300ea(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).framebufferRenderbuffer(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4));
}
export function __wbg_framebufferTexture2D_8adf6bdfc3c56dee(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4), arg5);
}
export function __wbg_framebufferTexture2D_c283e928186aa542(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4), arg5);
}
export function __wbg_framebufferTextureLayer_c8328828c8d5eb60(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).framebufferTextureLayer(arg1 >>> 0, arg2 >>> 0, getObject(arg3), arg4, arg5);
}
export function __wbg_framebufferTextureMultiviewOVR_16d049b41d692b91(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).framebufferTextureMultiviewOVR(arg1 >>> 0, arg2 >>> 0, getObject(arg3), arg4, arg5, arg6);
}
export function __wbg_frontFace_027e2ec7a7bc347c(arg0, arg1) {
    getObject(arg0).frontFace(arg1 >>> 0);
}
export function __wbg_frontFace_d4a6507ad2939b5c(arg0, arg1) {
    getObject(arg0).frontFace(arg1 >>> 0);
}
export function __wbg_fullscreenElement_25b445e2961e68ba(arg0) {
    const ret = getObject(arg0).fullscreenElement;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_getBoundingClientRect_b5c8c34d07878818(arg0) {
    const ret = getObject(arg0).getBoundingClientRect();
    return addHeapObject(ret);
}
export function __wbg_getBufferSubData_4fc54b4fbb1462d7(arg0, arg1, arg2, arg3) {
    getObject(arg0).getBufferSubData(arg1 >>> 0, arg2, getObject(arg3));
}
export function __wbg_getCoalescedEvents_21492912fd0145ec(arg0) {
    const ret = getObject(arg0).getCoalescedEvents;
    return addHeapObject(ret);
}
export function __wbg_getCoalescedEvents_8d19e426e1461e96(arg0) {
    const ret = getObject(arg0).getCoalescedEvents();
    return addHeapObject(ret);
}
export function __wbg_getComputedStyle_2d1f9dfe4ee7e0b9() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).getComputedStyle(getObject(arg1));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments); }
export function __wbg_getContext_2a5764d48600bc43() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments); }
export function __wbg_getContext_b28d2db7bd648242() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2), getObject(arg3));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments); }
export function __wbg_getContext_de810d9f187f29ca() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2), getObject(arg3));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments); }
export function __wbg_getExtension_3c0cb5ae01bb4b17() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).getExtension(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments); }
export function __wbg_getGamepads_5bcfb8576d477d73() { return handleError(function (arg0) {
    const ret = getObject(arg0).getGamepads();
    return addHeapObject(ret);
}, arguments); }
export function __wbg_getIndexedParameter_ca1693c768bc4934() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).getIndexedParameter(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
}, arguments); }
export function __wbg_getOwnPropertyDescriptor_03ccfd856865081b(arg0, arg1) {
    const ret = Object.getOwnPropertyDescriptor(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_getParameter_1ecb910cfdd21f88() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).getParameter(arg1 >>> 0);
    return addHeapObject(ret);
}, arguments); }
export function __wbg_getParameter_2e1f97ecaab76274() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).getParameter(arg1 >>> 0);
    return addHeapObject(ret);
}, arguments); }
export function __wbg_getProgramInfoLog_2ffa30e3abb8b5c2(arg0, arg1, arg2) {
    const ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_getProgramInfoLog_dbfda4b6e7eb1b37(arg0, arg1, arg2) {
    const ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_getProgramParameter_43fbc6d2613c08b3(arg0, arg1, arg2) {
    const ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
}
export function __wbg_getProgramParameter_92e4540ca9da06b2(arg0, arg1, arg2) {
    const ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
}
export function __wbg_getPropertyValue_d6911b2a1f9acba9() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = getObject(arg1).getPropertyValue(getStringFromWasm0(arg2, arg3));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments); }
export function __wbg_getQueryParameter_5d6af051438ae479(arg0, arg1, arg2) {
    const ret = getObject(arg0).getQueryParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
}
export function __wbg_getRandomValues_9c5c1b115e142bb8() { return handleError(function (arg0, arg1) {
    globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
}, arguments); }
export function __wbg_getShaderInfoLog_9991e9e77b0c6805(arg0, arg1, arg2) {
    const ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_getShaderInfoLog_9e0b96da4b13ae49(arg0, arg1, arg2) {
    const ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_getShaderParameter_786fd84f85720ca8(arg0, arg1, arg2) {
    const ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
}
export function __wbg_getShaderParameter_afa4a3dd9dd397c1(arg0, arg1, arg2) {
    const ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
}
export function __wbg_getSupportedExtensions_57142a6b598d7787(arg0) {
    const ret = getObject(arg0).getSupportedExtensions();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_getSupportedProfiles_1f728bc32003c4d0(arg0) {
    const ret = getObject(arg0).getSupportedProfiles();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_getSyncParameter_7d11ab875b41617e(arg0, arg1, arg2) {
    const ret = getObject(arg0).getSyncParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
}
export function __wbg_getUniformBlockIndex_1ee7e922e6d96d7e(arg0, arg1, arg2, arg3) {
    const ret = getObject(arg0).getUniformBlockIndex(getObject(arg1), getStringFromWasm0(arg2, arg3));
    return ret;
}
export function __wbg_getUniformLocation_71c070e6644669ad(arg0, arg1, arg2, arg3) {
    const ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_getUniformLocation_d06b3a5b3c60e95c(arg0, arg1, arg2, arg3) {
    const ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_get_9b94d73e6221f75c(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
}
export function __wbg_has_d4e53238966c12b6() { return handleError(function (arg0, arg1) {
    const ret = Reflect.has(getObject(arg0), getObject(arg1));
    return ret;
}, arguments); }
export function __wbg_height_c2027cf67d1c9b11(arg0) {
    const ret = getObject(arg0).height;
    return ret;
}
export function __wbg_id_e4749bded4876c88(arg0, arg1) {
    const ret = getObject(arg1).id;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_includes_32215c836f1cd3fb(arg0, arg1, arg2) {
    const ret = getObject(arg0).includes(getObject(arg1), arg2);
    return ret;
}
export function __wbg_index_254eff7dac36f09f(arg0) {
    const ret = getObject(arg0).index;
    return ret;
}
export function __wbg_inlineSize_3e4e7e8c813884fd(arg0) {
    const ret = getObject(arg0).inlineSize;
    return ret;
}
export function __wbg_instanceof_DomException_99c177193e554b75(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof DOMException;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_instanceof_HtmlCanvasElement_3f2f6e1edb1c9792(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLCanvasElement;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_instanceof_Response_ee1d54d79ae41977(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Response;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_instanceof_WebGl2RenderingContext_4a08a94517ed5240(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof WebGL2RenderingContext;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_instanceof_Window_ed49b2db8df90359(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Window;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
}
export function __wbg_invalidateFramebuffer_b17b7e1da3051745() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).invalidateFramebuffer(arg1 >>> 0, getObject(arg2));
}, arguments); }
export function __wbg_isIntersecting_6807d592d68e059e(arg0) {
    const ret = getObject(arg0).isIntersecting;
    return ret;
}
export function __wbg_isSecureContext_1e186b850f07cfb3(arg0) {
    const ret = getObject(arg0).isSecureContext;
    return ret;
}
export function __wbg_is_f29129f676e5410c(arg0, arg1) {
    const ret = Object.is(getObject(arg0), getObject(arg1));
    return ret;
}
export function __wbg_key_d41e8e825e6bb0e9(arg0, arg1) {
    const ret = getObject(arg1).key;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_length_32ed9a279acd054c(arg0) {
    const ret = getObject(arg0).length;
    return ret;
}
export function __wbg_length_35a7bace40f36eac(arg0) {
    const ret = getObject(arg0).length;
    return ret;
}
export function __wbg_linkProgram_6600dd2c0863bbfd(arg0, arg1) {
    getObject(arg0).linkProgram(getObject(arg1));
}
export function __wbg_linkProgram_be6b825cf66d177b(arg0, arg1) {
    getObject(arg0).linkProgram(getObject(arg1));
}
export function __wbg_location_22bcb1a188a96eb1(arg0) {
    const ret = getObject(arg0).location;
    return ret;
}
export function __wbg_log_0cc1b7768397bcfe(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.log(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3), getStringFromWasm0(arg4, arg5), getStringFromWasm0(arg6, arg7));
    } finally {
        wasm.__wbindgen_export4(deferred0_0, deferred0_1, 1);
    }
}
export function __wbg_log_cb9e190acc5753fb(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.log(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_export4(deferred0_0, deferred0_1, 1);
    }
}
export function __wbg_mapping_1b290f968173c826(arg0) {
    const ret = getObject(arg0).mapping;
    return (__wbindgen_enum_GamepadMappingType.indexOf(ret) + 1 || 3) - 1;
}
export function __wbg_mark_7438147ce31e9d4b(arg0, arg1) {
    performance.mark(getStringFromWasm0(arg0, arg1));
}
export function __wbg_matchMedia_91d4fc9729dc3c84() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).matchMedia(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments); }
export function __wbg_matches_4b5c22bd830f7bb3(arg0) {
    const ret = getObject(arg0).matches;
    return ret;
}
export function __wbg_maxChannelCount_70d101457dda9e24(arg0) {
    const ret = getObject(arg0).maxChannelCount;
    return ret;
}
export function __wbg_measure_fb7825c11612c823() { return handleError(function (arg0, arg1, arg2, arg3) {
    let deferred0_0;
    let deferred0_1;
    let deferred1_0;
    let deferred1_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        deferred1_0 = arg2;
        deferred1_1 = arg3;
        performance.measure(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
    } finally {
        wasm.__wbindgen_export4(deferred0_0, deferred0_1, 1);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}, arguments); }
export function __wbg_media_7bcde781569bca4c(arg0, arg1) {
    const ret = getObject(arg1).media;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_message_0b2b0298a231b0d4(arg0, arg1) {
    const ret = getObject(arg1).message;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_metaKey_374999c340f70626(arg0) {
    const ret = getObject(arg0).metaKey;
    return ret;
}
export function __wbg_metaKey_67113fb40365d736(arg0) {
    const ret = getObject(arg0).metaKey;
    return ret;
}
export function __wbg_movementX_ff6524e06bc35b6a(arg0) {
    const ret = getObject(arg0).movementX;
    return ret;
}
export function __wbg_movementY_4cec81d9850ad239(arg0) {
    const ret = getObject(arg0).movementY;
    return ret;
}
export function __wbg_navigator_43be698ba96fc088(arg0) {
    const ret = getObject(arg0).navigator;
    return addHeapObject(ret);
}
export function __wbg_new_2e2be9617c4407d5() { return handleError(function (arg0) {
    const ret = new ResizeObserver(getObject(arg0));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_new_361308b2356cecd0() {
    const ret = new Object();
    return addHeapObject(ret);
}
export function __wbg_new_3eb36ae241fe6f44() {
    const ret = new Array();
    return addHeapObject(ret);
}
export function __wbg_new_4f8f3c123e474358() { return handleError(function (arg0, arg1) {
    const ret = new Worker(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_new_6f0524fbfa300c47() { return handleError(function () {
    const ret = new MessageChannel();
    return addHeapObject(ret);
}, arguments); }
export function __wbg_new_8a6f238a6ece86ea() {
    const ret = new Error();
    return addHeapObject(ret);
}
export function __wbg_new_8c6e67a40cee1f83() { return handleError(function (arg0) {
    const ret = new IntersectionObserver(getObject(arg0));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_new_b949e7f56150a5d1() { return handleError(function () {
    const ret = new AbortController();
    return addHeapObject(ret);
}, arguments); }
export function __wbg_new_dd2b680c8bf6ae29(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
}
export function __wbg_new_de07934a2f24c2ec(arg0, arg1, arg2, arg3) {
    const ret = new RegExp(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
    return addHeapObject(ret);
}
export function __wbg_new_ed07138f2680efbf() { return handleError(function () {
    const ret = new Image();
    return addHeapObject(ret);
}, arguments); }
export function __wbg_new_no_args_1c7c842f08d00ebb(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}
export function __wbg_new_with_context_options_a4af9766d02e5169() { return handleError(function (arg0) {
    const ret = new lAudioContext(getObject(arg0));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_new_with_str_sequence_and_options_9b8b0bee99ec6b0f() { return handleError(function (arg0, arg1) {
    const ret = new Blob(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_new_with_u8_clamped_array_f2b5767e0116d2f8() { return handleError(function (arg0, arg1, arg2) {
    const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0);
    return addHeapObject(ret);
}, arguments); }
export function __wbg_now_2c95c9de01293173(arg0) {
    const ret = getObject(arg0).now();
    return ret;
}
export function __wbg_now_a3af9a2f4bbaa4d1() {
    const ret = Date.now();
    return ret;
}
export function __wbg_observe_1ae37077cf10b11b(arg0, arg1, arg2) {
    getObject(arg0).observe(getObject(arg1), getObject(arg2));
}
export function __wbg_observe_2a9d63459970a2c1(arg0, arg1) {
    getObject(arg0).observe(getObject(arg1));
}
export function __wbg_observe_b9abc08d6d829e56(arg0, arg1) {
    getObject(arg0).observe(getObject(arg1));
}
export function __wbg_of_9ab14f9d4bfb5040(arg0, arg1) {
    const ret = Array.of(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_of_f915f7cd925b21a5(arg0) {
    const ret = Array.of(getObject(arg0));
    return addHeapObject(ret);
}
export function __wbg_offsetX_cb6a38e6f23cb4a6(arg0) {
    const ret = getObject(arg0).offsetX;
    return ret;
}
export function __wbg_offsetY_43e21941c5c1f8bf(arg0) {
    const ret = getObject(arg0).offsetY;
    return ret;
}
export function __wbg_performance_7a3ffd0b17f663ad(arg0) {
    const ret = getObject(arg0).performance;
    return addHeapObject(ret);
}
export function __wbg_persisted_de98357e1aaf6546(arg0) {
    const ret = getObject(arg0).persisted;
    return ret;
}
export function __wbg_pixelStorei_2a65936c11b710fe(arg0, arg1, arg2) {
    getObject(arg0).pixelStorei(arg1 >>> 0, arg2);
}
export function __wbg_pixelStorei_f7cc498f52d523f1(arg0, arg1, arg2) {
    getObject(arg0).pixelStorei(arg1 >>> 0, arg2);
}
export function __wbg_play_63bc12f42e16af91(arg0) {
    getObject(arg0).play();
}
export function __wbg_pointerId_466b1bdcaf2fe835(arg0) {
    const ret = getObject(arg0).pointerId;
    return ret;
}
export function __wbg_pointerType_ba53c6f18634a26d(arg0, arg1) {
    const ret = getObject(arg1).pointerType;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_polygonOffset_24a8059deb03be92(arg0, arg1, arg2) {
    getObject(arg0).polygonOffset(arg1, arg2);
}
export function __wbg_polygonOffset_4b3158d8ed028862(arg0, arg1, arg2) {
    getObject(arg0).polygonOffset(arg1, arg2);
}
export function __wbg_port1_6251ddc5cf5c9287(arg0) {
    const ret = getObject(arg0).port1;
    return addHeapObject(ret);
}
export function __wbg_port2_b2a294b0ede1e13c(arg0) {
    const ret = getObject(arg0).port2;
    return addHeapObject(ret);
}
export function __wbg_postMessage_46eeeef39934b448() { return handleError(function (arg0, arg1) {
    getObject(arg0).postMessage(getObject(arg1));
}, arguments); }
export function __wbg_postMessage_e45c89e4826cf2ef() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).postMessage(getObject(arg1), getObject(arg2));
}, arguments); }
export function __wbg_postTask_41d93e93941e4a3d(arg0, arg1, arg2) {
    const ret = getObject(arg0).postTask(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}
export function __wbg_pressed_95e62758ea924dc3(arg0) {
    const ret = getObject(arg0).pressed;
    return ret;
}
export function __wbg_pressure_f01a99684f7a6cf3(arg0) {
    const ret = getObject(arg0).pressure;
    return ret;
}
export function __wbg_preventDefault_cdcfcd7e301b9702(arg0) {
    getObject(arg0).preventDefault();
}
export function __wbg_prototype_c28bca39c45aba9b() {
    const ret = ResizeObserverEntry.prototype;
    return addHeapObject(ret);
}
export function __wbg_prototypesetcall_bdcdcc5842e4d77d(arg0, arg1, arg2) {
    Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), getObject(arg2));
}
export function __wbg_push_8ffdcb2063340ba5(arg0, arg1) {
    const ret = getObject(arg0).push(getObject(arg1));
    return ret;
}
export function __wbg_queryCounterEXT_b578f07c30420446(arg0, arg1, arg2) {
    getObject(arg0).queryCounterEXT(getObject(arg1), arg2 >>> 0);
}
export function __wbg_querySelector_c3b0df2d58eec220() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).querySelector(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments); }
export function __wbg_queueMicrotask_0aa0a927f78f5d98(arg0) {
    const ret = getObject(arg0).queueMicrotask;
    return addHeapObject(ret);
}
export function __wbg_queueMicrotask_5bb536982f78a56f(arg0) {
    queueMicrotask(getObject(arg0));
}
export function __wbg_queueMicrotask_885fd8605352e25d(arg0, arg1) {
    getObject(arg0).queueMicrotask(getObject(arg1));
}
export function __wbg_readBuffer_9eb461d6857295f0(arg0, arg1) {
    getObject(arg0).readBuffer(arg1 >>> 0);
}
export function __wbg_readPixels_55b18304384e073d() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    getObject(arg0).readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, getObject(arg7));
}, arguments); }
export function __wbg_readPixels_6ea8e288a8673282() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    getObject(arg0).readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, arg7);
}, arguments); }
export function __wbg_readPixels_95b2464a7bb863a2() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    getObject(arg0).readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, getObject(arg7));
}, arguments); }
export function __wbg_removeEventListener_e63328781a5b9af9() { return handleError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
}, arguments); }
export function __wbg_removeListener_e2a199028636dcf5() { return handleError(function (arg0, arg1) {
    getObject(arg0).removeListener(getObject(arg1));
}, arguments); }
export function __wbg_removeProperty_a0d2ff8a76ffd2b1() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = getObject(arg1).removeProperty(getStringFromWasm0(arg2, arg3));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments); }
export function __wbg_renderbufferStorageMultisample_bc0ae08a7abb887a(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).renderbufferStorageMultisample(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
}
export function __wbg_renderbufferStorage_1bc02383614b76b2(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
}
export function __wbg_renderbufferStorage_6348154d30979c44(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).renderbufferStorage(arg1 >>> 0, arg2 >>> 0, arg3, arg4);
}
export function __wbg_repeat_375aae5c5c6a0258(arg0) {
    const ret = getObject(arg0).repeat;
    return ret;
}
export function __wbg_requestAnimationFrame_43682f8e1c5e5348() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
    return ret;
}, arguments); }
export function __wbg_requestFullscreen_86fc6cdb76000482(arg0) {
    const ret = getObject(arg0).requestFullscreen;
    return addHeapObject(ret);
}
export function __wbg_requestFullscreen_9f0611438eb929cf(arg0) {
    const ret = getObject(arg0).requestFullscreen();
    return addHeapObject(ret);
}
export function __wbg_requestIdleCallback_1b8d644ff564208f(arg0) {
    const ret = getObject(arg0).requestIdleCallback;
    return addHeapObject(ret);
}
export function __wbg_requestIdleCallback_c9c643f8210d435b() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).requestIdleCallback(getObject(arg1));
    return ret;
}, arguments); }
export function __wbg_requestPointerLock_f619fbb4f5d11204(arg0) {
    getObject(arg0).requestPointerLock();
}
export function __wbg_resolve_002c4b7d9d8f6b64(arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
}
export function __wbg_resume_7995ba29b6bb4edb() { return handleError(function (arg0) {
    const ret = getObject(arg0).resume();
    return addHeapObject(ret);
}, arguments); }
export function __wbg_revokeObjectURL_ba5712ef5af8bc9a() { return handleError(function (arg0, arg1) {
    URL.revokeObjectURL(getStringFromWasm0(arg0, arg1));
}, arguments); }
export function __wbg_samplerParameterf_f070d2b69b1e2d46(arg0, arg1, arg2, arg3) {
    getObject(arg0).samplerParameterf(getObject(arg1), arg2 >>> 0, arg3);
}
export function __wbg_samplerParameteri_8e4c4bcead0ee669(arg0, arg1, arg2, arg3) {
    getObject(arg0).samplerParameteri(getObject(arg1), arg2 >>> 0, arg3);
}
export function __wbg_scheduler_48482a9974eeacbd(arg0) {
    const ret = getObject(arg0).scheduler;
    return addHeapObject(ret);
}
export function __wbg_scheduler_5156bb61cc1cf589(arg0) {
    const ret = getObject(arg0).scheduler;
    return addHeapObject(ret);
}
export function __wbg_scissor_2ff8f18f05a6d408(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).scissor(arg1, arg2, arg3, arg4);
}
export function __wbg_scissor_b870b1434a9c25b4(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).scissor(arg1, arg2, arg3, arg4);
}
export function __wbg_setAttribute_cc8e4c8a2a008508() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
}, arguments); }
export function __wbg_setPointerCapture_420db6f6826eb74b() { return handleError(function (arg0, arg1) {
    getObject(arg0).setPointerCapture(arg1);
}, arguments); }
export function __wbg_setProperty_cbb25c4e74285b39() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
}, arguments); }
export function __wbg_setTimeout_681abd84926a4da3() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).setTimeout(getObject(arg1));
    return ret;
}, arguments); }
export function __wbg_setTimeout_eff32631ea138533() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
    return ret;
}, arguments); }
export function __wbg_set_6cb8631f80447a67() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    return ret;
}, arguments); }
export function __wbg_set_box_73d3355c6f95f24d(arg0, arg1) {
    getObject(arg0).box = __wbindgen_enum_ResizeObserverBoxOptions[arg1];
}
export function __wbg_set_buffer_730f06ef4e703eef(arg0, arg1) {
    getObject(arg0).buffer = getObject(arg1);
}
export function __wbg_set_channelCount_cb06e47d0ad98056(arg0, arg1) {
    getObject(arg0).channelCount = arg1 >>> 0;
}
export function __wbg_set_cursor_1424d80cf8ee0da5(arg0, arg1, arg2) {
    getObject(arg0).cursor = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_duration_91a970f42915997a(arg0, arg1) {
    getObject(arg0).duration = arg1;
}
export function __wbg_set_height_b386c0f603610637(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
}
export function __wbg_set_height_f21f985387070100(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
}
export function __wbg_set_iterations_ef8a200dad4272f0(arg0, arg1) {
    getObject(arg0).iterations = arg1;
}
export function __wbg_set_onended_af6372ae2d93a17d(arg0, arg1) {
    getObject(arg0).onended = getObject(arg1);
}
export function __wbg_set_onmessage_0e1ffb1c0d91d2ad(arg0, arg1) {
    getObject(arg0).onmessage = getObject(arg1);
}
export function __wbg_set_premultiply_alpha_c83ecfe43359687b(arg0, arg1) {
    getObject(arg0).premultiplyAlpha = __wbindgen_enum_PremultiplyAlpha[arg1];
}
export function __wbg_set_sample_rate_e24e368385a5a022(arg0, arg1) {
    getObject(arg0).sampleRate = arg1;
}
export function __wbg_set_src_55abd261cc86df86(arg0, arg1, arg2) {
    getObject(arg0).src = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_type_148de20768639245(arg0, arg1, arg2) {
    getObject(arg0).type = getStringFromWasm0(arg1, arg2);
}
export function __wbg_set_width_7f07715a20503914(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
}
export function __wbg_set_width_d60bc4f2f20c56a4(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
}
export function __wbg_shaderSource_32425cfe6e5a1e52(arg0, arg1, arg2, arg3) {
    getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
}
export function __wbg_shaderSource_8f4bda03f70359df(arg0, arg1, arg2, arg3) {
    getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
}
export function __wbg_shiftKey_5558a3288542c985(arg0) {
    const ret = getObject(arg0).shiftKey;
    return ret;
}
export function __wbg_shiftKey_564be91ec842bcc4(arg0) {
    const ret = getObject(arg0).shiftKey;
    return ret;
}
export function __wbg_signal_d1285ecab4ebc5ad(arg0) {
    const ret = getObject(arg0).signal;
    return addHeapObject(ret);
}
export function __wbg_stack_0ed75d68575b0f3c(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbg_start_76c52471f3287ae0() { return handleError(function (arg0, arg1) {
    getObject(arg0).start(arg1);
}, arguments); }
export function __wbg_start_ffb4b426b1e661bd(arg0) {
    getObject(arg0).start();
}
export function __wbg_static_accessor_GLOBAL_12837167ad935116() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_static_accessor_GLOBAL_THIS_e628e89ab3b1c95f() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_static_accessor_SELF_a621d3dfbb60d0ce() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_static_accessor_WINDOW_f8727f0cf888e0bd() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_status_89d7e803db911ee7(arg0) {
    const ret = getObject(arg0).status;
    return ret;
}
export function __wbg_stencilFuncSeparate_10d043d0af14366f(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).stencilFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3, arg4 >>> 0);
}
export function __wbg_stencilFuncSeparate_1798f5cca257f313(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).stencilFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3, arg4 >>> 0);
}
export function __wbg_stencilMaskSeparate_28d53625c02d9c7f(arg0, arg1, arg2) {
    getObject(arg0).stencilMaskSeparate(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_stencilMaskSeparate_c24c1a28b8dd8a63(arg0, arg1, arg2) {
    getObject(arg0).stencilMaskSeparate(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_stencilMask_0eca090c4c47f8f7(arg0, arg1) {
    getObject(arg0).stencilMask(arg1 >>> 0);
}
export function __wbg_stencilMask_732dcc5aada10e4c(arg0, arg1) {
    getObject(arg0).stencilMask(arg1 >>> 0);
}
export function __wbg_stencilOpSeparate_4657523b1d3b184f(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).stencilOpSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
}
export function __wbg_stencilOpSeparate_de257f3c29e604cd(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).stencilOpSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
}
export function __wbg_stringify_8d1cc6ff383e8bae() { return handleError(function (arg0) {
    const ret = JSON.stringify(getObject(arg0));
    return addHeapObject(ret);
}, arguments); }
export function __wbg_style_0b7c9bd318f8b807(arg0) {
    const ret = getObject(arg0).style;
    return addHeapObject(ret);
}
export function __wbg_texImage2D_087ef94df78081f0() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
}, arguments); }
export function __wbg_texImage2D_13414a4692836804() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texImage2D_e71049312f3172d9() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
}, arguments); }
export function __wbg_texImage3D_2082006a8a9b28a7() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    getObject(arg0).texImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8 >>> 0, arg9 >>> 0, arg10);
}, arguments); }
export function __wbg_texImage3D_bd2b0bd2cfcdb278() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    getObject(arg0).texImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8 >>> 0, arg9 >>> 0, getObject(arg10));
}, arguments); }
export function __wbg_texParameteri_0d45be2c88d6bad8(arg0, arg1, arg2, arg3) {
    getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
}
export function __wbg_texParameteri_ec937d2161018946(arg0, arg1, arg2, arg3) {
    getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
}
export function __wbg_texStorage2D_9504743abf5a986a(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).texStorage2D(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
}
export function __wbg_texStorage3D_e9e1b58fee218abe(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).texStorage3D(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5, arg6);
}
export function __wbg_texSubImage2D_117d29278542feb0() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
}, arguments); }
export function __wbg_texSubImage2D_19ae4cadb809f264() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
}, arguments); }
export function __wbg_texSubImage2D_5d270af600a7fc4a() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
}, arguments); }
export function __wbg_texSubImage2D_bd034db2e58c352c() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
}, arguments); }
export function __wbg_texSubImage2D_bf72e56edeeed376() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
}, arguments); }
export function __wbg_texSubImage2D_d17a39cdec4a3495() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
}, arguments); }
export function __wbg_texSubImage2D_e193f1d28439217c() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
}, arguments); }
export function __wbg_texSubImage2D_edf5bd70fda3feaf() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9);
}, arguments); }
export function __wbg_texSubImage3D_1102c12a20bf56d5() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    getObject(arg0).texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, getObject(arg11));
}, arguments); }
export function __wbg_texSubImage3D_18d7f3c65567c885() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    getObject(arg0).texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, getObject(arg11));
}, arguments); }
export function __wbg_texSubImage3D_3b653017c4c5d721() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    getObject(arg0).texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, getObject(arg11));
}, arguments); }
export function __wbg_texSubImage3D_45591e5655d1ed5c() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    getObject(arg0).texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, getObject(arg11));
}, arguments); }
export function __wbg_texSubImage3D_47643556a8a4bf86() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    getObject(arg0).texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, getObject(arg11));
}, arguments); }
export function __wbg_texSubImage3D_59b8e24fb05787aa() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    getObject(arg0).texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, arg11);
}, arguments); }
export function __wbg_texSubImage3D_eff5cd6ab84f44ee() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) {
    getObject(arg0).texSubImage3D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9 >>> 0, arg10 >>> 0, getObject(arg11));
}, arguments); }
export function __wbg_then_0d9fe2c7b1857d32(arg0, arg1, arg2) {
    const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}
export function __wbg_then_b9e7b3b5f1a9e1b5(arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
}
export function __wbg_toBlob_debb4a0169b10d99() { return handleError(function (arg0, arg1) {
    getObject(arg0).toBlob(getObject(arg1));
}, arguments); }
export function __wbg_transferFromImageBitmap_b9299bbc2ec148e5(arg0, arg1) {
    getObject(arg0).transferFromImageBitmap(getObject(arg1));
}
export function __wbg_uniform1f_b500ede5b612bea2(arg0, arg1, arg2) {
    getObject(arg0).uniform1f(getObject(arg1), arg2);
}
export function __wbg_uniform1f_c148eeaf4b531059(arg0, arg1, arg2) {
    getObject(arg0).uniform1f(getObject(arg1), arg2);
}
export function __wbg_uniform1i_9f3f72dbcb98ada9(arg0, arg1, arg2) {
    getObject(arg0).uniform1i(getObject(arg1), arg2);
}
export function __wbg_uniform1i_e9aee4b9e7fe8c4b(arg0, arg1, arg2) {
    getObject(arg0).uniform1i(getObject(arg1), arg2);
}
export function __wbg_uniform1ui_a0f911ff174715d0(arg0, arg1, arg2) {
    getObject(arg0).uniform1ui(getObject(arg1), arg2 >>> 0);
}
export function __wbg_uniform2fv_04c304b93cbf7f55(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform2fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform2fv_2fb47cfe06330cc7(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform2fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform2iv_095baf208f172131(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform2iv(getObject(arg1), getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform2iv_ccf2ed44ac8e602e(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform2iv(getObject(arg1), getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform2uiv_3030d7e769f5e82a(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform2uiv(getObject(arg1), getArrayU32FromWasm0(arg2, arg3));
}
export function __wbg_uniform3fv_aa35ef21e14d5469(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform3fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform3fv_c0872003729939a5(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform3fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform3iv_6aa2b0791e659d14(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform3iv(getObject(arg1), getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform3iv_e912f444d4ff8269(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform3iv(getObject(arg1), getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform3uiv_86941e7eeb8ee0a3(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform3uiv(getObject(arg1), getArrayU32FromWasm0(arg2, arg3));
}
export function __wbg_uniform4f_71ec75443e58cecc(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
}
export function __wbg_uniform4f_f6b5e2024636033a(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
}
export function __wbg_uniform4fv_498bd80dc5aa16ff(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform4fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform4fv_e6c73702e9a3be5c(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform4fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
}
export function __wbg_uniform4iv_375332584c65e61b(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform4iv(getObject(arg1), getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform4iv_8a8219fda39dffd5(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform4iv(getObject(arg1), getArrayI32FromWasm0(arg2, arg3));
}
export function __wbg_uniform4uiv_046ee400bb80547d(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform4uiv(getObject(arg1), getArrayU32FromWasm0(arg2, arg3));
}
export function __wbg_uniformBlockBinding_1cf9fd2c49adf0f3(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniformBlockBinding(getObject(arg1), arg2 >>> 0, arg3 >>> 0);
}
export function __wbg_uniformMatrix2fv_24430076c7afb5e3(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix2fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix2fv_e2806601f5b95102(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix2fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix2x3fv_a377326104a8faf4(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix2x3fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix2x4fv_b7a4d810e7a1cf7d(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix2x4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix3fv_6f822361173d8046(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix3fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix3fv_b94a764c63aa6468(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix3fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix3x2fv_69a4cf0ce5b09f8b(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix3x2fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix3x4fv_cc72e31a1baaf9c9(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix3x4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix4fv_0e724dbebd372526(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix4fv_923b55ad503fdc56(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix4x2fv_8c9fb646f3b90b63(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix4x2fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_uniformMatrix4x3fv_ee0bed9a1330400d(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix4x3fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}
export function __wbg_unobserve_b4eb8d945252124f(arg0, arg1) {
    getObject(arg0).unobserve(getObject(arg1));
}
export function __wbg_useProgram_e82c1a5f87d81579(arg0, arg1) {
    getObject(arg0).useProgram(getObject(arg1));
}
export function __wbg_useProgram_fe720ade4d3b6edb(arg0, arg1) {
    getObject(arg0).useProgram(getObject(arg1));
}
export function __wbg_userAgentData_f7b0e61c05c54315(arg0) {
    const ret = getObject(arg0).userAgentData;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_userAgent_34463fd660ba4a2a() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg1).userAgent;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments); }
export function __wbg_value_6cd76ff54b88ed85(arg0) {
    const ret = getObject(arg0).value;
    return ret;
}
export function __wbg_vertexAttribDivisorANGLE_eaa3c29423ea6da4(arg0, arg1, arg2) {
    getObject(arg0).vertexAttribDivisorANGLE(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_vertexAttribDivisor_744c0ca468594894(arg0, arg1, arg2) {
    getObject(arg0).vertexAttribDivisor(arg1 >>> 0, arg2 >>> 0);
}
export function __wbg_vertexAttribIPointer_b9020d0c2e759912(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).vertexAttribIPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
}
export function __wbg_vertexAttribPointer_75f6ff47f6c9f8cb(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
}
export function __wbg_vertexAttribPointer_adbd1853cce679ad(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
}
export function __wbg_viewport_174ae1c2209344ae(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).viewport(arg1, arg2, arg3, arg4);
}
export function __wbg_viewport_df236eac68bc7467(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).viewport(arg1, arg2, arg3, arg4);
}
export function __wbg_visibilityState_43b7b74940e07d22(arg0) {
    const ret = getObject(arg0).visibilityState;
    return (__wbindgen_enum_VisibilityState.indexOf(ret) + 1 || 3) - 1;
}
export function __wbg_webkitExitFullscreen_85426cef5e755dfa(arg0) {
    getObject(arg0).webkitExitFullscreen();
}
export function __wbg_webkitFullscreenElement_a9ca38b7214d1567(arg0) {
    const ret = getObject(arg0).webkitFullscreenElement;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}
export function __wbg_webkitRequestFullscreen_23664c63833ff0e5(arg0) {
    getObject(arg0).webkitRequestFullscreen();
}
export function __wbg_width_7444cca5dfea0645(arg0) {
    const ret = getObject(arg0).width;
    return ret;
}
export function __wbg_x_95222ef76724a332(arg0) {
    const ret = getObject(arg0).x;
    return ret;
}
export function __wbg_y_0b4e7ff7d5c0a5d7(arg0) {
    const ret = getObject(arg0).y;
    return ret;
}
export function __wbindgen_cast_0000000000000001(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 79281, function: Function { arguments: [], shim_idx: 79282, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_154077, __wasm_bindgen_func_elem_154110);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000002(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 80333, function: Function { arguments: [Externref], shim_idx: 80334, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_158455, __wasm_bindgen_func_elem_158469);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000003(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 81430, function: Function { arguments: [NamedExternref("Array<any>"), NamedExternref("ResizeObserver")], shim_idx: 81434, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_164403, __wasm_bindgen_func_elem_164626);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000004(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 81430, function: Function { arguments: [NamedExternref("Array<any>")], shim_idx: 81431, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_164403, __wasm_bindgen_func_elem_164623);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000005(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 81430, function: Function { arguments: [NamedExternref("Event")], shim_idx: 81431, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_164403, __wasm_bindgen_func_elem_164623);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000006(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 81430, function: Function { arguments: [NamedExternref("FocusEvent")], shim_idx: 81431, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_164403, __wasm_bindgen_func_elem_164623);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000007(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 81430, function: Function { arguments: [NamedExternref("KeyboardEvent")], shim_idx: 81431, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_164403, __wasm_bindgen_func_elem_164623);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000008(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 81430, function: Function { arguments: [NamedExternref("PageTransitionEvent")], shim_idx: 81431, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_164403, __wasm_bindgen_func_elem_164623);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000009(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 81430, function: Function { arguments: [NamedExternref("PointerEvent")], shim_idx: 81431, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_164403, __wasm_bindgen_func_elem_164623);
    return addHeapObject(ret);
}
export function __wbindgen_cast_000000000000000a(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 81430, function: Function { arguments: [NamedExternref("WheelEvent")], shim_idx: 81431, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_164403, __wasm_bindgen_func_elem_164623);
    return addHeapObject(ret);
}
export function __wbindgen_cast_000000000000000b(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 81430, function: Function { arguments: [Option(NamedExternref("Blob"))], shim_idx: 81433, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_164403, __wasm_bindgen_func_elem_164624);
    return addHeapObject(ret);
}
export function __wbindgen_cast_000000000000000c(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 81430, function: Function { arguments: [], shim_idx: 81432, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.__wasm_bindgen_func_elem_164403, __wasm_bindgen_func_elem_164622);
    return addHeapObject(ret);
}
export function __wbindgen_cast_000000000000000d(arg0) {
    // Cast intrinsic for `F64 -> Externref`.
    const ret = arg0;
    return addHeapObject(ret);
}
export function __wbindgen_cast_000000000000000e(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(F32)) -> NamedExternref("Float32Array")`.
    const ret = getArrayF32FromWasm0(arg0, arg1);
    return addHeapObject(ret);
}
export function __wbindgen_cast_000000000000000f(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(I16)) -> NamedExternref("Int16Array")`.
    const ret = getArrayI16FromWasm0(arg0, arg1);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000010(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(I32)) -> NamedExternref("Int32Array")`.
    const ret = getArrayI32FromWasm0(arg0, arg1);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000011(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(I8)) -> NamedExternref("Int8Array")`.
    const ret = getArrayI8FromWasm0(arg0, arg1);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000012(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(U16)) -> NamedExternref("Uint16Array")`.
    const ret = getArrayU16FromWasm0(arg0, arg1);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000013(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(U32)) -> NamedExternref("Uint32Array")`.
    const ret = getArrayU32FromWasm0(arg0, arg1);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000014(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
    const ret = getArrayU8FromWasm0(arg0, arg1);
    return addHeapObject(ret);
}
export function __wbindgen_cast_0000000000000015(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
}
export function __wbindgen_object_clone_ref(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
}
export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
}
const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : (typeof webkitAudioContext !== 'undefined' ? webkitAudioContext : undefined));
function __wasm_bindgen_func_elem_154110(arg0, arg1) {
    wasm.__wasm_bindgen_func_elem_154110(arg0, arg1);
}

function __wasm_bindgen_func_elem_164622(arg0, arg1) {
    wasm.__wasm_bindgen_func_elem_164622(arg0, arg1);
}

function __wasm_bindgen_func_elem_158469(arg0, arg1, arg2) {
    wasm.__wasm_bindgen_func_elem_158469(arg0, arg1, addHeapObject(arg2));
}

function __wasm_bindgen_func_elem_164623(arg0, arg1, arg2) {
    wasm.__wasm_bindgen_func_elem_164623(arg0, arg1, addHeapObject(arg2));
}

function __wasm_bindgen_func_elem_164624(arg0, arg1, arg2) {
    wasm.__wasm_bindgen_func_elem_164624(arg0, arg1, isLikeNone(arg2) ? 0 : addHeapObject(arg2));
}

function __wasm_bindgen_func_elem_164626(arg0, arg1, arg2, arg3) {
    wasm.__wasm_bindgen_func_elem_164626(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}


const __wbindgen_enum_GamepadMappingType = ["", "standard"];


const __wbindgen_enum_PremultiplyAlpha = ["none", "premultiply", "default"];


const __wbindgen_enum_ResizeObserverBoxOptions = ["border-box", "content-box", "device-pixel-content-box"];


const __wbindgen_enum_VisibilityState = ["hidden", "visible"];

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayI16FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
}

function getArrayI32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayI8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getInt8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getArrayU16FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getClampedArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ClampedArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

let cachedInt16ArrayMemory0 = null;
function getInt16ArrayMemory0() {
    if (cachedInt16ArrayMemory0 === null || cachedInt16ArrayMemory0.byteLength === 0) {
        cachedInt16ArrayMemory0 = new Int16Array(wasm.memory.buffer);
    }
    return cachedInt16ArrayMemory0;
}

let cachedInt32ArrayMemory0 = null;
function getInt32ArrayMemory0() {
    if (cachedInt32ArrayMemory0 === null || cachedInt32ArrayMemory0.byteLength === 0) {
        cachedInt32ArrayMemory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32ArrayMemory0;
}

let cachedInt8ArrayMemory0 = null;
function getInt8ArrayMemory0() {
    if (cachedInt8ArrayMemory0 === null || cachedInt8ArrayMemory0.byteLength === 0) {
        cachedInt8ArrayMemory0 = new Int8Array(wasm.memory.buffer);
    }
    return cachedInt8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint16ArrayMemory0 = null;
function getUint16ArrayMemory0() {
    if (cachedUint16ArrayMemory0 === null || cachedUint16ArrayMemory0.byteLength === 0) {
        cachedUint16ArrayMemory0 = new Uint16Array(wasm.memory.buffer);
    }
    return cachedUint16ArrayMemory0;
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedUint8ClampedArrayMemory0 = null;
function getUint8ClampedArrayMemory0() {
    if (cachedUint8ClampedArrayMemory0 === null || cachedUint8ClampedArrayMemory0.byteLength === 0) {
        cachedUint8ClampedArrayMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachedUint8ClampedArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export3(addHeapObject(e));
    }
}

let heap = new Array(128).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
