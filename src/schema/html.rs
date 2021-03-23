pub mod attrs {
    // mustUseProperty: ['checked', 'multiple', 'muted', 'selected'],
    crate::define_attrs!(
        (abbr, "abbr", "abbr", String),
        (accept, "accept", "accept", CommaSeparated),
        (accept_charset, "accept-charset", "acceptCharset", SpaceSeparated),
        (access_key, "accesskey", "accessKey", SpaceSeparated),
        (action, "action", "action", String),
        (allow, "allow", "allow", String),
        (allow_full_screen, "allowfullscreen", "allowFullScreen", Bool),
        (allow_payment_request, "allowpaymentrequest", "allowPaymentRequest", Bool),
        (allow_user_media, "allowusermedia", "allowUserMedia", Bool),
        (alt, "alt", "alt", String),
        (as_, "as", "as", String),
        (async_, "async", "async", Bool),
        (auto_capitalize, "autocapitalize", "autoCapitalize", String),
        (auto_complete, "autocomplete", "autoComplete", SpaceSeparated),
        (auto_focus, "autofocus", "autoFocus", Bool),
        (auto_play, "autoplay", "autoPlay", Bool),
        (capture, "capture", "capture", Bool),
        (charset, "charset", "charSet", String),
        (checked, "checked", "checked", Bool),
        (cite, "cite", "cite", String),
        (class_name, "class", "className", SpaceSeparated),
        (cols, "cols", "cols", Number),
        (colspan, "colspan", "colSpan", String),
        (content, "content", "content", String),
        (contente_ditable, "contenteditable", "contentEditable", BooleanIsh),
        (controls, "controls", "controls", Bool),
        (controls_list, "controlslist", "controlsList", SpaceSeparated),
        (coords, "coords", "coords", CommaSeparated),
        (cross_origin, "crossorigin", "crossOrigin", String),
        (data, "data", "data", String),
        (date_time, "datetime", "dateTime", String),
        (decoding, "decoding", "decoding", String),
        (default, "default", "default", Bool),
        (defer, "defer", "defer", Bool),
        (dir, "dir", "dir", String),
        (dir_name, "dirname", "dirName", String),
        (disabled, "disabled", "disabled", Bool),
        (download, "download", "download", OverloadedBool),
        (draggable, "draggable", "draggable", BooleanIsh),
        (enc_type, "enctype", "encType", String),
        (enter_key_hint, "enterkeyhint", "enterKeyHint", String),
        (form, "form", "form", String),
        (form_action, "formaction", "formAction", String),
        (form_enc_type, "formenctype", "formEncType", String),
        (form_method, "formmethod", "formMethod", String),
        (form_no_validate, "formnovalidate", "formNoValidate", Bool),
        (form_target, "formtarget", "formTarget", String),
        (headers, "headers", "headers", SpaceSeparated),
        (height, "height", "height", Number),
        (hidden, "hidden", "hidden", Bool),
        (high, "high", "high", Number),
        (href, "href", "href", String),
        (href_lang, "hreflang", "hrefLang", String),
        (html_for, "for", "htmlFor", SpaceSeparated),
        (http_equiv, "http-equiv", "httpEquiv", SpaceSeparated),
        (id, "id", "id", String),
        (image_sizes, "imagesizes", "imageSizes", String),
        (image_src_set, "imagesrcset", "imageSrcSet", CommaSeparated),
        (input_mode, "inputmode", "inputMode", String),
        (integrity, "integrity", "integrity", String),
        (is, "is", "is", String),
        (is_map, "ismap", "isMap", Bool),
        (item_id, "itemid", "itemId", String),
        (item_prop, "itemprop", "itemProp", SpaceSeparated),
        (item_ref, "itemref", "itemRef", SpaceSeparated),
        (item_scope, "itemscope", "itemScope", Bool),
        (item_type, "itemtype", "itemType", SpaceSeparated),
        (kind, "kind", "kind", String),
        (label, "label", "label", String),
        (lang, "lang", "lang", String),
        (language, "language", "language", String),
        (list, "list", "list", String),
        (loading, "loading", "loading", String),
        (loop_, "loop", "loop", Bool),
        (low, "low", "low", Number),
        (manifest, "manifest", "manifest", String),
        (max, "max", "max", String),
        (max_length, "maxlength", "maxLength", Number),
        (media, "media", "media", String),
        (method, "method", "method", String),
        (min, "min", "min", String),
        (min_length, "minlength", "minLength", Number),
        (multiple, "multiple", "multiple", Bool),
        (muted, "muted", "muted", Bool),
        (name, "name", "name", String),
        (nonce, "nonce", "nonce", String),
        (no_module, "nomodule", "noModule", Bool),
        (no_validate, "novalidate", "noValidate", Bool),
        (on_abort, "onabort", "onAbort", String),
        (on_after_print, "onafterprint", "onAfterPrint", String),
        (on_aux_click, "onauxclick", "onAuxClick", String),
        (on_before_print, "onbeforeprint", "onBeforePrint", String),
        (on_before_unload, "onbeforeunload", "onBeforeUnload", String),
        (on_blur, "onblur", "onBlur", String),
        (on_cancel, "oncancel", "onCancel", String),
        (on_can_play, "oncanplay", "onCanPlay", String),
        (on_can_play_through, "oncanplaythrough", "onCanPlayThrough", String),
        (on_change, "onchange", "onChange", String),
        (on_click, "onclick", "onClick", String),
        (on_close, "onclose", "onClose", String),
        (on_context_menu, "oncontextmenu", "onContextMenu", String),
        (on_copy, "oncopy", "onCopy", String),
        (on_cue_change, "oncuechange", "onCueChange", String),
        (on_cut, "oncut", "onCut", String),
        (on_dbl_click, "ondblclick", "onDblClick", String),
        (on_drag, "ondrag", "onDrag", String),
        (on_drag_end, "ondragend", "onDragEnd", String),
        (on_drag_enter, "ondragenter", "onDragEnter", String),
        (on_drag_exit, "ondragexit", "onDragExit", String),
        (on_drag_leave, "ondragleave", "onDragLeave", String),
        (on_drag_over, "ondragover", "onDragOver", String),
        (on_drag_start, "ondragstart", "onDragStart", String),
        (on_drop, "ondrop", "onDrop", String),
        (on_duration_change, "ondurationchange", "onDurationChange", String),
        (on_emptied, "onemptied", "onEmptied", String),
        (on_ended, "onended", "onEnded", String),
        (on_error, "onerror", "onError", String),
        (on_focus, "onfocus", "onFocus", String),
        (on_form_data, "onformdata", "onFormData", String),
        (on_hash_change, "onhashchange", "onHashChange", String),
        (on_input, "oninput", "onInput", String),
        (on_invalid, "oninvalid", "onInvalid", String),
        (on_key_down, "onkeydown", "onKeyDown", String),
        (on_key_press, "onkeypress", "onKeyPress", String),
        (on_key_up, "onkeyup", "onKeyUp", String),
        (on_language_change, "onlanguagechange", "onLanguageChange", String),
        (on_load, "onload", "onLoad", String),
        (on_loaded_data, "onloadeddata", "onLoadedData", String),
        (on_loaded_metadata, "onloadedmetadata", "onLoadedMetadata", String),
        (on_load_end, "onloadend", "onLoadEnd", String),
        (on_load_start, "onloadstart", "onLoadStart", String),
        (on_message, "onmessage", "onMessage", String),
        (on_message_error, "onmessageerror", "onMessageError", String),
        (on_mouse_down, "onmousedown", "onMouseDown", String),
        (on_mouse_enter, "onmouseenter", "onMouseEnter", String),
        (on_mouse_leave, "onmouseleave", "onMouseLeave", String),
        (on_mouse_move, "onmousemove", "onMouseMove", String),
        (on_mouse_out, "onmouseout", "onMouseOut", String),
        (on_mouse_over, "onmouseover", "onMouseOver", String),
        (on_mouse_up, "onmouseup", "onMouseUp", String),
        (on_offline, "onoffline", "onOffline", String),
        (on_online, "ononline", "onOnline", String),
        (on_page_hide, "onpagehide", "onPageHide", String),
        (on_page_show, "onpageshow", "onPageShow", String),
        (on_paste, "onpaste", "onPaste", String),
        (on_pause, "onpause", "onPause", String),
        (on_play, "onplay", "onPlay", String),
        (on_playing, "onplaying", "onPlaying", String),
        (on_pop_state, "onpopstate", "onPopState", String),
        (on_progress, "onprogress", "onProgress", String),
        (on_rate_change, "onratechange", "onRateChange", String),
        (on_rejection_handled, "onrejectionhandled", "onRejectionHandled", String),
        (on_reset, "onreset", "onReset", String),
        (on_resize, "onresize", "onResize", String),
        (on_scroll, "onscroll", "onScroll", String),
        (on_security_policy_violation, "onsecuritypolicyviolation", "onSecurityPolicyViolation", String),
        (on_seeked, "onseeked", "onSeeked", String),
        (on_seeking, "onseeking", "onSeeking", String),
        (on_select, "onselect", "onSelect", String),
        (on_slot_change, "onslotchange", "onSlotChange", String),
        (on_stalled, "onstalled", "onStalled", String),
        (on_storage, "onstorage", "onStorage", String),
        (on_submit, "onsubmit", "onSubmit", String),
        (on_suspend, "onsuspend", "onSuspend", String),
        (on_time_update, "ontimeupdate", "onTimeUpdate", String),
        (on_toggle, "ontoggle", "onToggle", String),
        (on_unhandled_rejection, "onunhandledrejection", "onUnhandledRejection", String),
        (on_unload, "onunload", "onUnload", String),
        (on_volume_change, "onvolumechange", "onVolumeChange", String),
        (on_waiting, "onwaiting", "onWaiting", String),
        (on_wheel, "onwheel", "onWheel", String),
        (open, "open", "open", Bool),
        (optimum, "optimum", "optimum", Number),
        (pattern, "pattern", "pattern", String),
        (ping, "ping", "ping", SpaceSeparated),
        (placeholder, "placeholder", "placeholder", String),
        (plays_inline, "playsinline", "playsInline", Bool),
        (poster, "poster", "poster", String),
        (preload, "preload", "preload", String),
        (read_only, "readonly", "readOnly", Bool),
        (referrer_policy, "referrerpolicy", "referrerPolicy", String),
        (rel, "rel", "rel", SpaceSeparated),
        (required, "required", "required", Bool),
        (reversed, "reversed", "reversed", Bool),
        (rows, "rows", "rows", Number),
        (row_span, "rowspan", "rowSpan", Number),
        (sandbox, "sandbox", "sandbox", SpaceSeparated),
        (scope, "scope", "scope", String),
        (scoped, "scoped", "scoped", Bool),
        (seamless, "seamless", "seamless", Bool),
        (selected, "selected", "selected", Bool),
        (shape, "shape", "shape", String),
        (size, "size", "size", Number),
        (sizes, "sizes", "sizes", String),
        (slot, "slot", "slot", String),
        (span, "span", "span", Number),
        (spell_check, "spellcheck", "spellCheck", BooleanIsh),
        (src, "src", "src", String),
        (src_doc, "srcdoc", "srcDoc", String),
        (src_lang, "srclang", "srcLang", String),
        (src_set, "srcset", "srcSet", CommaSeparated),
        (start, "start", "start", Number),
        (step, "step", "step", String),
        (style, "style", "style", String),
        (tab_index, "tabindex", "tabIndex", Number),
        (target, "target", "target", String),
        (title, "title", "title", String),
        (translate, "translate", "translate", String),
        (type_, "type", "type", String),
        (type_must_match, "typemustmatch", "typeMustMatch", Bool),
        (use_map, "usemap", "useMap", String),
        (value, "value", "value", BooleanIsh),
        (width, "width", "width", Number),
        (wrap, "wrap", "wrap", String),

        // Legacy.
        // See: https://html.spec.whatwg.org/#other-elements,-attributes-and-apis
        (align, "align", "align", String), // Several. Use CSS `text-align` instead,
        (a_link, "alink", "aLink", String), // `<body>`. Use CSS `a:active {color}` instead
        (archive, "archive", "archive", SpaceSeparated), // `<object>`. List of URIs to archives
        (axis, "axis", "axis", String), // `<td>` and `<th>`. Use `scope` on `<th>`
        (background, "background", "background", String), // `<body>`. Use CSS `background-image` instead
        (bg_color, "bgcolor", "bgColor", String), // `<body>` and table elements. Use CSS `background-color` instead
        (border, "border", "border", Number), // `<table>`. Use CSS `border-width` instead,
        (border_color, "bordercolor", "borderColor", String), // `<table>`. Use CSS `border-color` instead,
        (bottom_margin, "bottommargin", "bottomMargin", Number), // `<body>`
        (cell_padding, "cellpadding", "cellPadding", String), // `<table>`
        (cell_spacing, "cellspacing", "cellSpacing", String), // `<table>`
        (char, "char", "char", String), // Several table elements. When `align=char`, sets the character to align on
        (char_off, "charoff", "charOff", String), // Several table elements. When `char`, offsets the alignment
        (class_id, "classid", "classId", String), // `<object>`
        (clear, "clear", "clear", String), // `<br>`. Use CSS `clear` instead
        (code, "code", "code", String), // `<object>`
        (code_base, "codebase", "codeBase", String), // `<object>`
        (code_type, "codetype", "codeType", String), // `<object>`
        (color, "color", "color", String), // `<font>` and `<hr>`. Use CSS instead
        (compact, "compact", "compact", Bool), // Lists. Use CSS to reduce space between items instead
        (declare, "declare", "declare", Bool), // `<object>`
        (event, "event", "event", String), // `<script>`
        (face, "face", "face", String), // `<font>`. Use CSS instead
        (frame, "frame", "frame", String), // `<table>`
        (frame_border, "frameborder", "frameBorder", String), // `<iframe>`. Use CSS `border` instead
        (h_space, "hspace", "hSpace", Number), // `<img>` and `<object>`
        (left_margin, "leftmargin", "leftMargin", Number), // `<body>`
        (link, "link", "link", String), // `<body>`. Use CSS `a:link {color: *}` instead
        (long_desc, "longdesc", "longDesc", String), // `<frame>`, `<iframe>`, and `<img>`. Use an `<a>`
        (low_src, "lowsrc", "lowSrc", String), // `<img>`. Use a `<picture>`
        (margin_height, "marginheight", "marginHeight", Number), // `<body>`
        (margin_width, "marginwidth", "marginWidth", Number), // `<body>`
        (no_resize, "noresize", "noResize", Bool), // `<frame>`
        (no_href, "nohref", "noHref", Bool), // `<area>`. Use no href instead of an explicit `nohref`
        (no_shade, "noshade", "noShade", Bool), // `<hr>`. Use background-color and height instead of borders
        (no_wrap, "nowrap", "noWrap", Bool), // `<td>` and `<th>`
        (object, "object", "object", String), // `<applet>`
        (profile, "profile", "profile", String), // `<head>`
        (prompt, "prompt", "prompt", String), // `<isindex>`
        (rev, "rev", "rev", String), // `<link>`
        (right_margin, "rightmargin", "rightMargin", Number), // `<body>`
        (rules, "rules", "rules", String), // `<table>`
        (scheme, "scheme", "scheme", String), // `<meta>`
        (scrolling, "scrolling", "scrolling", BooleanIsh), // `<frame>`. Use overflow in the child context
        (standby, "standby", "standby", String), // `<object>`
        (summary, "summary", "summary", String), // `<table>`
        (text, "text", "text", String), // `<body>`. Use CSS `color` instead
        (top_margin, "topmargin", "topMargin", Number), // `<body>`
        (value_type, "valuetype", "valueType", String), // `<param>`
        (version, "version", "version", String), // `<html>`. Use a doctype.
        (v_align, "valign", "vAlign", String), // Several. Use CSS `vertical-align` instead
        (v_link, "vlink", "vLink", String), // `<body>`. Use CSS `a:visited {color}` instead
        (v_space, "vspace", "vSpace", Number), // `<img>` and `<object>`

        // Non-standard Properties.
        (allow_transparency, "allowtransparency", "allowTransparency", String),
        (auto_correct, "autocorrect", "autoCorrect", String),
        (auto_save, "autosave", "autoSave", String),
        (disable_picture_in_picture, "disablepictureinpicture", "disablePictureInPicture", Bool),
        (disable_remote_playback, "disableremoteplayback", "disableRemotePlayback", Bool),
        (prefix, "prefix", "prefix", String),
        (property, "property", "property", String),
        (results, "results", "results", Number),
        (security, "security", "security", String),
        (unselectable, "unselectable", "unselectable", String)
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
