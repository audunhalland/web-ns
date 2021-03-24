pub mod attrs {
    // mustUseProperty: ['checked', 'multiple', 'muted', 'selected'],
    crate::define_attrs!(
        (abbr, "abbr", "abbr", One, String),
        (accept, "accept", "accept", CommaSeparated, String),
        (accept_charset, "accept-charset", "acceptCharset", SpaceSeparated, String),
        (access_key, "accesskey", "accessKey", SpaceSeparated, String),
        (action, "action", "action", One, String),
        (allow, "allow", "allow", One, String),
        (allow_full_screen, "allowfullscreen", "allowFullScreen", One, Bool),
        (allow_payment_request, "allowpaymentrequest", "allowPaymentRequest", One, Bool),
        (allow_user_media, "allowusermedia", "allowUserMedia", One, Bool),
        (alt, "alt", "alt", One, String),
        (as_, "as", "as", One, String),
        (async_, "async", "async", One, Bool),
        (auto_capitalize, "autocapitalize", "autoCapitalize", One, String),
        (auto_complete, "autocomplete", "autoComplete", SpaceSeparated, String),
        (auto_focus, "autofocus", "autoFocus", One, Bool),
        (auto_play, "autoplay", "autoPlay", One, Bool),
        (capture, "capture", "capture", One, Bool),
        (charset, "charset", "charSet", One, String),
        (checked, "checked", "checked", One, Bool),
        (cite, "cite", "cite", One, String),
        (class_name, "class", "className", SpaceSeparated, String),
        (cols, "cols", "cols", One, Number),
        (colspan, "colspan", "colSpan", One, String),
        (content, "content", "content", One, String),
        (contente_ditable, "contenteditable", "contentEditable", One, BoolOrEmptyString),
        (controls, "controls", "controls", One, Bool),
        (controls_list, "controlslist", "controlsList", SpaceSeparated, String),
        (coords, "coords", "coords", CommaSeparated, String),
        (cross_origin, "crossorigin", "crossOrigin", One, String),
        (data, "data", "data", One, String),
        (date_time, "datetime", "dateTime", One, String),
        (decoding, "decoding", "decoding", One, String),
        (default, "default", "default", One, Bool),
        (defer, "defer", "defer", One, Bool),
        (dir, "dir", "dir", One, String),
        (dir_name, "dirname", "dirName", One, String),
        (disabled, "disabled", "disabled", One, Bool),
        (download, "download", "download", One, OverloadedBool),
        (draggable, "draggable", "draggable", One, BooleanIsh),
        (enc_type, "enctype", "encType", One, String),
        (enter_key_hint, "enterkeyhint", "enterKeyHint", One, String),
        (form, "form", "form", One, String),
        (form_action, "formaction", "formAction", One, String),
        (form_enc_type, "formenctype", "formEncType", One, String),
        (form_method, "formmethod", "formMethod", One, String),
        (form_no_validate, "formnovalidate", "formNoValidate", One, Bool),
        (form_target, "formtarget", "formTarget", One, String),
        (headers, "headers", "headers", SpaceSeparated, String),
        (height, "height", "height", One, Number),
        (hidden, "hidden", "hidden", One, Bool),
        (high, "high", "high", One, Number),
        (href, "href", "href", One, String),
        (href_lang, "hreflang", "hrefLang", One, String),
        (html_for, "for", "htmlFor", SpaceSeparated, String),
        (http_equiv, "http-equiv", "httpEquiv", SpaceSeparated, String),
        (id, "id", "id", One, String),
        (image_sizes, "imagesizes", "imageSizes", One, String),
        (image_src_set, "imagesrcset", "imageSrcSet", CommaSeparated, String),
        (input_mode, "inputmode", "inputMode", One, String),
        (integrity, "integrity", "integrity", One, String),
        (is, "is", "is", One, String),
        (is_map, "ismap", "isMap", One, Bool),
        (item_id, "itemid", "itemId", One, String),
        (item_prop, "itemprop", "itemProp", SpaceSeparated, String),
        (item_ref, "itemref", "itemRef", SpaceSeparated, String),
        (item_scope, "itemscope", "itemScope", One, Bool),
        (item_type, "itemtype", "itemType", SpaceSeparated, String),
        (kind, "kind", "kind", One, String),
        (label, "label", "label", One, String),
        (lang, "lang", "lang", One, String),
        (language, "language", "language", One, String),
        (list, "list", "list", One, String),
        (loading, "loading", "loading", One, String),
        (loop_, "loop", "loop", One, Bool),
        (low, "low", "low", One, Number),
        (manifest, "manifest", "manifest", One, String),
        (max, "max", "max", One, String),
        (max_length, "maxlength", "maxLength", One, Number),
        (media, "media", "media", One, String),
        (method, "method", "method", One, String),
        (min, "min", "min", One, String),
        (min_length, "minlength", "minLength", One, Number),
        (multiple, "multiple", "multiple", One, Bool),
        (muted, "muted", "muted", One, Bool),
        (name, "name", "name", One, String),
        (nonce, "nonce", "nonce", One, String),
        (no_module, "nomodule", "noModule", One, Bool),
        (no_validate, "novalidate", "noValidate", One, Bool),
        (on_abort, "onabort", "onAbort", One, String),
        (on_after_print, "onafterprint", "onAfterPrint", One, String),
        (on_aux_click, "onauxclick", "onAuxClick", One, String),
        (on_before_print, "onbeforeprint", "onBeforePrint", One, String),
        (on_before_unload, "onbeforeunload", "onBeforeUnload", One, String),
        (on_blur, "onblur", "onBlur", One, String),
        (on_cancel, "oncancel", "onCancel", One, String),
        (on_can_play, "oncanplay", "onCanPlay", One, String),
        (on_can_play_through, "oncanplaythrough", "onCanPlayThrough", One, String),
        (on_change, "onchange", "onChange", One, String),
        (on_click, "onclick", "onClick", One, String),
        (on_close, "onclose", "onClose", One, String),
        (on_context_menu, "oncontextmenu", "onContextMenu", One, String),
        (on_copy, "oncopy", "onCopy", One, String),
        (on_cue_change, "oncuechange", "onCueChange", One, String),
        (on_cut, "oncut", "onCut", One, String),
        (on_dbl_click, "ondblclick", "onDblClick", One, String),
        (on_drag, "ondrag", "onDrag", One, String),
        (on_drag_end, "ondragend", "onDragEnd", One, String),
        (on_drag_enter, "ondragenter", "onDragEnter", One, String),
        (on_drag_exit, "ondragexit", "onDragExit", One, String),
        (on_drag_leave, "ondragleave", "onDragLeave", One, String),
        (on_drag_over, "ondragover", "onDragOver", One, String),
        (on_drag_start, "ondragstart", "onDragStart", One, String),
        (on_drop, "ondrop", "onDrop", One, String),
        (on_duration_change, "ondurationchange", "onDurationChange", One, String),
        (on_emptied, "onemptied", "onEmptied", One, String),
        (on_ended, "onended", "onEnded", One, String),
        (on_error, "onerror", "onError", One, String),
        (on_focus, "onfocus", "onFocus", One, String),
        (on_form_data, "onformdata", "onFormData", One, String),
        (on_hash_change, "onhashchange", "onHashChange", One, String),
        (on_input, "oninput", "onInput", One, String),
        (on_invalid, "oninvalid", "onInvalid", One, String),
        (on_key_down, "onkeydown", "onKeyDown", One, String),
        (on_key_press, "onkeypress", "onKeyPress", One, String),
        (on_key_up, "onkeyup", "onKeyUp", One, String),
        (on_language_change, "onlanguagechange", "onLanguageChange", One, String),
        (on_load, "onload", "onLoad", One, String),
        (on_loaded_data, "onloadeddata", "onLoadedData", One, String),
        (on_loaded_metadata, "onloadedmetadata", "onLoadedMetadata", One, String),
        (on_load_end, "onloadend", "onLoadEnd", One, String),
        (on_load_start, "onloadstart", "onLoadStart", One, String),
        (on_message, "onmessage", "onMessage", One, String),
        (on_message_error, "onmessageerror", "onMessageError", One, String),
        (on_mouse_down, "onmousedown", "onMouseDown", One, String),
        (on_mouse_enter, "onmouseenter", "onMouseEnter", One, String),
        (on_mouse_leave, "onmouseleave", "onMouseLeave", One, String),
        (on_mouse_move, "onmousemove", "onMouseMove", One, String),
        (on_mouse_out, "onmouseout", "onMouseOut", One, String),
        (on_mouse_over, "onmouseover", "onMouseOver", One, String),
        (on_mouse_up, "onmouseup", "onMouseUp", One, String),
        (on_offline, "onoffline", "onOffline", One, String),
        (on_online, "ononline", "onOnline", One, String),
        (on_page_hide, "onpagehide", "onPageHide", One, String),
        (on_page_show, "onpageshow", "onPageShow", One, String),
        (on_paste, "onpaste", "onPaste", One, String),
        (on_pause, "onpause", "onPause", One, String),
        (on_play, "onplay", "onPlay", One, String),
        (on_playing, "onplaying", "onPlaying", One, String),
        (on_pop_state, "onpopstate", "onPopState", One, String),
        (on_progress, "onprogress", "onProgress", One, String),
        (on_rate_change, "onratechange", "onRateChange", One, String),
        (on_rejection_handled, "onrejectionhandled", "onRejectionHandled", One, String),
        (on_reset, "onreset", "onReset", One, String),
        (on_resize, "onresize", "onResize", One, String),
        (on_scroll, "onscroll", "onScroll", One, String),
        (on_security_policy_violation, "onsecuritypolicyviolation", "onSecurityPolicyViolation", One, String),
        (on_seeked, "onseeked", "onSeeked", One, String),
        (on_seeking, "onseeking", "onSeeking", One, String),
        (on_select, "onselect", "onSelect", One, String),
        (on_slot_change, "onslotchange", "onSlotChange", One, String),
        (on_stalled, "onstalled", "onStalled", One, String),
        (on_storage, "onstorage", "onStorage", One, String),
        (on_submit, "onsubmit", "onSubmit", One, String),
        (on_suspend, "onsuspend", "onSuspend", One, String),
        (on_time_update, "ontimeupdate", "onTimeUpdate", One, String),
        (on_toggle, "ontoggle", "onToggle", One, String),
        (on_unhandled_rejection, "onunhandledrejection", "onUnhandledRejection", One, String),
        (on_unload, "onunload", "onUnload", One, String),
        (on_volume_change, "onvolumechange", "onVolumeChange", One, String),
        (on_waiting, "onwaiting", "onWaiting", One, String),
        (on_wheel, "onwheel", "onWheel", One, String),
        (open, "open", "open", One, Bool),
        (optimum, "optimum", "optimum", One, Number),
        (pattern, "pattern", "pattern", One, String),
        (ping, "ping", "ping", SpaceSeparated, String),
        (placeholder, "placeholder", "placeholder", One, String),
        (plays_inline, "playsinline", "playsInline", One, Bool),
        (poster, "poster", "poster", One, String),
        (preload, "preload", "preload", One, String),
        (read_only, "readonly", "readOnly", One, Bool),
        (referrer_policy, "referrerpolicy", "referrerPolicy", One, String),
        (rel, "rel", "rel", SpaceSeparated, String),
        (required, "required", "required", One, Bool),
        (reversed, "reversed", "reversed", One, Bool),
        (rows, "rows", "rows", One, Number),
        (row_span, "rowspan", "rowSpan", One, Number),
        (sandbox, "sandbox", "sandbox", SpaceSeparated, String),
        (scope, "scope", "scope", One, String),
        (scoped, "scoped", "scoped", One, Bool),
        (seamless, "seamless", "seamless", One, Bool),
        (selected, "selected", "selected", One, Bool),
        (shape, "shape", "shape", One, String),
        (size, "size", "size", One, Number),
        (sizes, "sizes", "sizes", One, String),
        (slot, "slot", "slot", One, String),
        (span, "span", "span", One, Number),
        (spell_check, "spellcheck", "spellCheck", One, BooleanIsh),
        (src, "src", "src", One, String),
        (src_doc, "srcdoc", "srcDoc", One, String),
        (src_lang, "srclang", "srcLang", One, String),
        (src_set, "srcset", "srcSet", CommaSeparated, String),
        (start, "start", "start", One, Number),
        (step, "step", "step", One, String),
        (style, "style", "style", One, String),
        (tab_index, "tabindex", "tabIndex", One, Number),
        (target, "target", "target", One, String),
        (title, "title", "title", One, String),
        (translate, "translate", "translate", One, String),
        (type_, "type", "type", One, String),
        (type_must_match, "typemustmatch", "typeMustMatch", One, Bool),
        (use_map, "usemap", "useMap", One, String),
        (value, "value", "value", One, BooleanIsh),
        (width, "width", "width", One, Number),
        (wrap, "wrap", "wrap", One, String),

        // Legacy.
        // See: https://html.spec.whatwg.org/#other-elements,-attributes-and-apis
        (align, "align", "align", One, String), // Several. Use CSS `text-align` instead,
        (a_link, "alink", "aLink", One, String), // `<body>`. Use CSS `a:active {color}` instead
        (archive, "archive", "archive", SpaceSeparated, String), // `<object>`. List of URIs to archives
        (axis, "axis", "axis", One, String), // `<td>` and `<th>`. Use `scope` on `<th>`
        (background, "background", "background", One, String), // `<body>`. Use CSS `background-image` instead
        (bg_color, "bgcolor", "bgColor", One, String), // `<body>` and table elements. Use CSS `background-color` instead
        (border, "border", "border", One, Number), // `<table>`. Use CSS `border-width` instead,
        (border_color, "bordercolor", "borderColor", One, String), // `<table>`. Use CSS `border-color` instead,
        (bottom_margin, "bottommargin", "bottomMargin", One, Number), // `<body>`
        (cell_padding, "cellpadding", "cellPadding", One, String), // `<table>`
        (cell_spacing, "cellspacing", "cellSpacing", One, String), // `<table>`
        (char, "char", "char", One, String), // Several table elements. When `align=char`, sets the character to align on
        (char_off, "charoff", "charOff", One, String), // Several table elements. When `char`, offsets the alignment
        (class_id, "classid", "classId", One, String), // `<object>`
        (clear, "clear", "clear", One, String), // `<br>`. Use CSS `clear` instead
        (code, "code", "code", One, String), // `<object>`
        (code_base, "codebase", "codeBase", One, String), // `<object>`
        (code_type, "codetype", "codeType", One, String), // `<object>`
        (color, "color", "color", One, String), // `<font>` and `<hr>`. Use CSS instead
        (compact, "compact", "compact", One, Bool), // Lists. Use CSS to reduce space between items instead
        (declare, "declare", "declare", One, Bool), // `<object>`
        (event, "event", "event", One, String), // `<script>`
        (face, "face", "face", One, String), // `<font>`. Use CSS instead
        (frame, "frame", "frame", One, String), // `<table>`
        (frame_border, "frameborder", "frameBorder", One, String), // `<iframe>`. Use CSS `border` instead
        (h_space, "hspace", "hSpace", One, Number), // `<img>` and `<object>`
        (left_margin, "leftmargin", "leftMargin", One, Number), // `<body>`
        (link, "link", "link", One, String), // `<body>`. Use CSS `a:link {color: *}` instead
        (long_desc, "longdesc", "longDesc", One, String), // `<frame>`, `<iframe>`, and `<img>`. Use an `<a>`
        (low_src, "lowsrc", "lowSrc", One, String), // `<img>`. Use a `<picture>`
        (margin_height, "marginheight", "marginHeight", One, Number), // `<body>`
        (margin_width, "marginwidth", "marginWidth", One, Number), // `<body>`
        (no_resize, "noresize", "noResize", One, Bool), // `<frame>`
        (no_href, "nohref", "noHref", One, Bool), // `<area>`. Use no href instead of an explicit `nohref`
        (no_shade, "noshade", "noShade", One, Bool), // `<hr>`. Use background-color and height instead of borders
        (no_wrap, "nowrap", "noWrap", One, Bool), // `<td>` and `<th>`
        (object, "object", "object", One, String), // `<applet>`
        (profile, "profile", "profile", One, String), // `<head>`
        (prompt, "prompt", "prompt", One, String), // `<isindex>`
        (rev, "rev", "rev", One, String), // `<link>`
        (right_margin, "rightmargin", "rightMargin", One, Number), // `<body>`
        (rules, "rules", "rules", One, String), // `<table>`
        (scheme, "scheme", "scheme", One, String), // `<meta>`
        (scrolling, "scrolling", "scrolling", One, BooleanIsh), // `<frame>`. Use overflow in the child context
        (standby, "standby", "standby", One, String), // `<object>`
        (summary, "summary", "summary", One, String), // `<table>`
        (text, "text", "text", One, String), // `<body>`. Use CSS `color` instead
        (top_margin, "topmargin", "topMargin", One, Number), // `<body>`
        (value_type, "valuetype", "valueType", One, String), // `<param>`
        (version, "version", "version", One, String), // `<html>`. Use a doctype.
        (v_align, "valign", "vAlign", One, String), // Several. Use CSS `vertical-align` instead
        (v_link, "vlink", "vLink", One, String), // `<body>`. Use CSS `a:visited {color}` instead
        (v_space, "vspace", "vSpace", One, Number), // `<img>` and `<object>`

        // Non-standard Properties.
        (allow_transparency, "allowtransparency", "allowTransparency", One, String),
        (auto_correct, "autocorrect", "autoCorrect", One, String),
        (auto_save, "autosave", "autoSave", One, String),
        (disable_picture_in_picture, "disablepictureinpicture", "disablePictureInPicture", One, Bool),
        (disable_remote_playback, "disableremoteplayback", "disableRemotePlayback", One, Bool),
        (prefix, "prefix", "prefix", One, String),
        (property, "property", "property", One, String),
        (results, "results", "results", One, Number),
        (security, "security", "security", One, String),
        (unselectable, "unselectable", "unselectable", One, String)
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn attribute_by_name() {
        let internal_attr = super::attrs::internal_attr_by_name("alink").unwrap();
        assert_eq!(internal_attr.attribute, "alink");
        assert_eq!(internal_attr.property, "aLink");
    }
}
