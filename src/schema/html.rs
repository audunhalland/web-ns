pub mod attrs {
    // mustUseProperty: 'checked', 'multiple', 'muted', 'selected'],

    crate::define_attrs!(
        (abbr, "abbr", "abbr", STRING),
        (accept, "accept", "accept", COMMA_SEP | STRING),
        (
            accept_charset,
            "accept-charset",
            "acceptCharset",
            SPACE_SEP | STRING
        ),
        (access_key, "accesskey", "accessKey", SPACE_SEP | STRING),
        (action, "action", "action", STRING),
        (allow, "allow", "allow", STRING),
        (
            allow_full_screen,
            "allowfullscreen",
            "allowFullScreen",
            BOOL
        ),
        (
            allow_payment_request,
            "allowpaymentrequest",
            "allowPaymentRequest",
            BOOL
        ),
        (allow_user_media, "allowusermedia", "allowUserMedia", BOOL),
        (alt, "alt", "alt", STRING),
        (as_, "as", "as", STRING),
        (async_, "async", "async", BOOL),
        (auto_capitalize, "autocapitalize", "autoCapitalize", STRING),
        (
            auto_complete,
            "autocomplete",
            "autoComplete",
            SPACE_SEP | STRING
        ),
        (auto_focus, "autofocus", "autoFocus", BOOL),
        (auto_play, "autoplay", "autoPlay", BOOL),
        (capture, "capture", "capture", BOOL),
        (charset, "charset", "charSet", STRING),
        (checked, "checked", "checked", BOOL),
        (cite, "cite", "cite", STRING),
        (class_name, "class", "className", SPACE_SEP | STRING),
        (cols, "cols", "cols", NUMBER),
        (colspan, "colspan", "colSpan", STRING),
        (content, "content", "content", STRING),
        (
            contente_ditable,
            "contenteditable",
            "contentEditable",
            TRUE | EMPTY_STRING | FALSE
        ),
        (controls, "controls", "controls", BOOL),
        (
            controls_list,
            "controlslist",
            "controlsList",
            SPACE_SEP | STRING
        ),
        (coords, "coords", "coords", COMMA_SEP | STRING),
        (cross_origin, "crossorigin", "crossOrigin", STRING),
        (data, "data", "data", STRING),
        (date_time, "datetime", "dateTime", STRING),
        (decoding, "decoding", "decoding", STRING),
        (default, "default", "default", BOOL),
        (defer, "defer", "defer", BOOL),
        (dir, "dir", "dir", STRING),
        (dir_name, "dirname", "dirName", STRING),
        (disabled, "disabled", "disabled", BOOL),
        (download, "download", "download", BOOL | STRING),
        (draggable, "draggable", "draggable", TRUE | FALSE),
        (enc_type, "enctype", "encType", STRING),
        (enter_key_hint, "enterkeyhint", "enterKeyHint", STRING),
        (form, "form", "form", STRING),
        (form_action, "formaction", "formAction", STRING),
        (form_enc_type, "formenctype", "formEncType", STRING),
        (form_method, "formmethod", "formMethod", STRING),
        (form_no_validate, "formnovalidate", "formNoValidate", BOOL),
        (form_target, "formtarget", "formTarget", STRING),
        (headers, "headers", "headers", SPACE_SEP | STRING),
        (height, "height", "height", NUMBER),
        (hidden, "hidden", "hidden", BOOL),
        (high, "high", "high", NUMBER),
        (href, "href", "href", STRING),
        (href_lang, "hreflang", "hrefLang", STRING),
        (html_for, "for", "htmlFor", SPACE_SEP | STRING),
        (http_equiv, "http-equiv", "httpEquiv", SPACE_SEP | STRING),
        (id, "id", "id", STRING),
        (image_sizes, "imagesizes", "imageSizes", STRING),
        (
            image_src_set,
            "imagesrcset",
            "imageSrcSet",
            COMMA_SEP | STRING
        ),
        (input_mode, "inputmode", "inputMode", STRING),
        (integrity, "integrity", "integrity", STRING),
        (is, "is", "is", STRING),
        (is_map, "ismap", "isMap", BOOL),
        (item_id, "itemid", "itemId", STRING),
        (item_prop, "itemprop", "itemProp", SPACE_SEP | STRING),
        (item_ref, "itemref", "itemRef", SPACE_SEP | STRING),
        (item_scope, "itemscope", "itemScope", BOOL),
        (item_type, "itemtype", "itemType", SPACE_SEP | STRING),
        (kind, "kind", "kind", STRING),
        (label, "label", "label", STRING),
        (lang, "lang", "lang", STRING),
        (language, "language", "language", STRING),
        (list, "list", "list", STRING),
        (loading, "loading", "loading", STRING),
        (loop_, "loop", "loop", BOOL),
        (low, "low", "low", NUMBER),
        (manifest, "manifest", "manifest", STRING),
        (max, "max", "max", STRING),
        (max_length, "maxlength", "maxLength", NUMBER),
        (media, "media", "media", STRING),
        (method, "method", "method", STRING),
        (min, "min", "min", STRING),
        (min_length, "minlength", "minLength", NUMBER),
        (multiple, "multiple", "multiple", BOOL),
        (muted, "muted", "muted", BOOL),
        (name, "name", "name", STRING),
        (nonce, "nonce", "nonce", STRING),
        (no_module, "nomodule", "noModule", BOOL),
        (no_validate, "novalidate", "noValidate", BOOL),
        (on_abort, "onabort", "onAbort", STRING),
        (on_after_print, "onafterprint", "onAfterPrint", STRING),
        (on_aux_click, "onauxclick", "onAuxClick", STRING),
        (on_before_print, "onbeforeprint", "onBeforePrint", STRING),
        (on_before_unload, "onbeforeunload", "onBeforeUnload", STRING),
        (on_blur, "onblur", "onBlur", STRING),
        (on_cancel, "oncancel", "onCancel", STRING),
        (on_can_play, "oncanplay", "onCanPlay", STRING),
        (
            on_can_play_through,
            "oncanplaythrough",
            "onCanPlayThrough",
            STRING
        ),
        (on_change, "onchange", "onChange", STRING),
        (on_click, "onclick", "onClick", STRING),
        (on_close, "onclose", "onClose", STRING),
        (on_context_menu, "oncontextmenu", "onContextMenu", STRING),
        (on_copy, "oncopy", "onCopy", STRING),
        (on_cue_change, "oncuechange", "onCueChange", STRING),
        (on_cut, "oncut", "onCut", STRING),
        (on_dbl_click, "ondblclick", "onDblClick", STRING),
        (on_drag, "ondrag", "onDrag", STRING),
        (on_drag_end, "ondragend", "onDragEnd", STRING),
        (on_drag_enter, "ondragenter", "onDragEnter", STRING),
        (on_drag_exit, "ondragexit", "onDragExit", STRING),
        (on_drag_leave, "ondragleave", "onDragLeave", STRING),
        (on_drag_over, "ondragover", "onDragOver", STRING),
        (on_drag_start, "ondragstart", "onDragStart", STRING),
        (on_drop, "ondrop", "onDrop", STRING),
        (
            on_duration_change,
            "ondurationchange",
            "onDurationChange",
            STRING
        ),
        (on_emptied, "onemptied", "onEmptied", STRING),
        (on_ended, "onended", "onEnded", STRING),
        (on_error, "onerror", "onError", STRING),
        (on_focus, "onfocus", "onFocus", STRING),
        (on_form_data, "onformdata", "onFormData", STRING),
        (on_hash_change, "onhashchange", "onHashChange", STRING),
        (on_input, "oninput", "onInput", STRING),
        (on_invalid, "oninvalid", "onInvalid", STRING),
        (on_key_down, "onkeydown", "onKeyDown", STRING),
        (on_key_press, "onkeypress", "onKeyPress", STRING),
        (on_key_up, "onkeyup", "onKeyUp", STRING),
        (
            on_language_change,
            "onlanguagechange",
            "onLanguageChange",
            STRING
        ),
        (on_load, "onload", "onLoad", STRING),
        (on_loaded_data, "onloadeddata", "onLoadedData", STRING),
        (
            on_loaded_metadata,
            "onloadedmetadata",
            "onLoadedMetadata",
            STRING
        ),
        (on_load_end, "onloadend", "onLoadEnd", STRING),
        (on_load_start, "onloadstart", "onLoadStart", STRING),
        (on_message, "onmessage", "onMessage", STRING),
        (on_message_error, "onmessageerror", "onMessageError", STRING),
        (on_mouse_down, "onmousedown", "onMouseDown", STRING),
        (on_mouse_enter, "onmouseenter", "onMouseEnter", STRING),
        (on_mouse_leave, "onmouseleave", "onMouseLeave", STRING),
        (on_mouse_move, "onmousemove", "onMouseMove", STRING),
        (on_mouse_out, "onmouseout", "onMouseOut", STRING),
        (on_mouse_over, "onmouseover", "onMouseOver", STRING),
        (on_mouse_up, "onmouseup", "onMouseUp", STRING),
        (on_offline, "onoffline", "onOffline", STRING),
        (on_online, "ononline", "onOnline", STRING),
        (on_page_hide, "onpagehide", "onPageHide", STRING),
        (on_page_show, "onpageshow", "onPageShow", STRING),
        (on_paste, "onpaste", "onPaste", STRING),
        (on_pause, "onpause", "onPause", STRING),
        (on_play, "onplay", "onPlay", STRING),
        (on_playing, "onplaying", "onPlaying", STRING),
        (on_pop_state, "onpopstate", "onPopState", STRING),
        (on_progress, "onprogress", "onProgress", STRING),
        (on_rate_change, "onratechange", "onRateChange", STRING),
        (
            on_rejection_handled,
            "onrejectionhandled",
            "onRejectionHandled",
            STRING
        ),
        (on_reset, "onreset", "onReset", STRING),
        (on_resize, "onresize", "onResize", STRING),
        (on_scroll, "onscroll", "onScroll", STRING),
        (
            on_security_policy_violation,
            "onsecuritypolicyviolation",
            "onSecurityPolicyViolation",
            STRING
        ),
        (on_seeked, "onseeked", "onSeeked", STRING),
        (on_seeking, "onseeking", "onSeeking", STRING),
        (on_select, "onselect", "onSelect", STRING),
        (on_slot_change, "onslotchange", "onSlotChange", STRING),
        (on_stalled, "onstalled", "onStalled", STRING),
        (on_storage, "onstorage", "onStorage", STRING),
        (on_submit, "onsubmit", "onSubmit", STRING),
        (on_suspend, "onsuspend", "onSuspend", STRING),
        (on_time_update, "ontimeupdate", "onTimeUpdate", STRING),
        (on_toggle, "ontoggle", "onToggle", STRING),
        (
            on_unhandled_rejection,
            "onunhandledrejection",
            "onUnhandledRejection",
            STRING
        ),
        (on_unload, "onunload", "onUnload", STRING),
        (on_volume_change, "onvolumechange", "onVolumeChange", STRING),
        (on_waiting, "onwaiting", "onWaiting", STRING),
        (on_wheel, "onwheel", "onWheel", STRING),
        (open, "open", "open", BOOL),
        (optimum, "optimum", "optimum", NUMBER),
        (pattern, "pattern", "pattern", STRING),
        (ping, "ping", "ping", SPACE_SEP | STRING),
        (placeholder, "placeholder", "placeholder", STRING),
        (plays_inline, "playsinline", "playsInline", BOOL),
        (poster, "poster", "poster", STRING),
        (preload, "preload", "preload", STRING),
        (read_only, "readonly", "readOnly", BOOL),
        (referrer_policy, "referrerpolicy", "referrerPolicy", STRING),
        (rel, "rel", "rel", SPACE_SEP | STRING),
        (required, "required", "required", BOOL),
        (reversed, "reversed", "reversed", BOOL),
        (rows, "rows", "rows", NUMBER),
        (row_span, "rowspan", "rowSpan", NUMBER),
        (sandbox, "sandbox", "sandbox", SPACE_SEP | STRING),
        (scope, "scope", "scope", STRING),
        (scoped, "scoped", "scoped", BOOL),
        (seamless, "seamless", "seamless", BOOL),
        (selected, "selected", "selected", BOOL),
        (shape, "shape", "shape", STRING),
        (size, "size", "size", NUMBER),
        (sizes, "sizes", "sizes", STRING),
        (slot, "slot", "slot", STRING),
        (span, "span", "span", NUMBER),
        (spell_check, "spellcheck", "spellCheck", TRUE | FALSE),
        (src, "src", "src", STRING),
        (src_doc, "srcdoc", "srcDoc", STRING),
        (src_lang, "srclang", "srcLang", STRING),
        (src_set, "srcset", "srcSet", COMMA_SEP | STRING),
        (start, "start", "start", NUMBER),
        (step, "step", "step", STRING),
        (style, "style", "style", STRING),
        (tab_index, "tabindex", "tabIndex", NUMBER),
        (target, "target", "target", STRING),
        (title, "title", "title", STRING),
        (translate, "translate", "translate", STRING),
        (type_, "type", "type", STRING),
        (type_must_match, "typemustmatch", "typeMustMatch", BOOL),
        (use_map, "usemap", "useMap", STRING),
        (value, "value", "value", TRUE | FALSE | STRING),
        (width, "width", "width", NUMBER),
        (wrap, "wrap", "wrap", STRING),
        // Legacy.
        // See: https://html.spec.whatwg.org/#other-elements,-attributes-and-apis
        (align, "align", "align", STRING), // Several. Use CSS `text-align` instead,
        (a_link, "alink", "aLink", STRING), // `<body>`. Use CSS `a:active {color}` instead
        (archive, "archive", "archive", SPACE_SEP | STRING), // `<object>`. List of URIs to archives
        (axis, "axis", "axis", STRING),    // `<td>` and `<th>`. Use `scope` on `<th>`
        (background, "background", "background", STRING), // `<body>`. Use CSS `background-image` instead
        (bg_color, "bgcolor", "bgColor", STRING), // `<body>` and table elements. Use CSS `background-color` instead
        (border, "border", "border", NUMBER),     // `<table>`. Use CSS `border-width` instead,
        (border_color, "bordercolor", "borderColor", STRING), // `<table>`. Use CSS `border-color` instead,
        (bottom_margin, "bottommargin", "bottomMargin", NUMBER), // `<body>`
        (cell_padding, "cellpadding", "cellPadding", STRING), // `<table>`
        (cell_spacing, "cellspacing", "cellSpacing", STRING), // `<table>`
        (char, "char", "char", STRING), // Several table elements. When `align=char`, sets the character to align on
        (char_off, "charoff", "charOff", STRING), // Several table elements. When `char`, offsets the alignment
        (class_id, "classid", "classId", STRING), // `<object>`
        (clear, "clear", "clear", STRING),        // `<br>`. Use CSS `clear` instead
        (code, "code", "code", STRING),           // `<object>`
        (code_base, "codebase", "codeBase", STRING), // `<object>`
        (code_type, "codetype", "codeType", STRING), // `<object>`
        (color, "color", "color", STRING),        // `<font>` and `<hr>`. Use CSS instead
        (compact, "compact", "compact", BOOL), // Lists. Use CSS to reduce space between items instead
        (declare, "declare", "declare", BOOL), // `<object>`
        (event, "event", "event", STRING),     // `<script>`
        (face, "face", "face", STRING),        // `<font>`. Use CSS instead
        (frame, "frame", "frame", STRING),     // `<table>`
        (frame_border, "frameborder", "frameBorder", STRING), // `<iframe>`. Use CSS `border` instead
        (h_space, "hspace", "hSpace", NUMBER),                // `<img>` and `<object>`
        (left_margin, "leftmargin", "leftMargin", NUMBER),    // `<body>`
        (link, "link", "link", STRING), // `<body>`. Use CSS `a:link {color: *}` instead
        (long_desc, "longdesc", "longDesc", STRING), // `<frame>`, `<iframe>`, and `<img>`. Use an `<a>`
        (low_src, "lowsrc", "lowSrc", STRING),       // `<img>`. Use a `<picture>`
        (margin_height, "marginheight", "marginHeight", NUMBER), // `<body>`
        (margin_width, "marginwidth", "marginWidth", NUMBER), // `<body>`
        (no_resize, "noresize", "noResize", BOOL),   // `<frame>`
        (no_href, "nohref", "noHref", BOOL), // `<area>`. Use no href instead of an explicit `nohref`
        (no_shade, "noshade", "noShade", BOOL), // `<hr>`. Use background-color and height instead of borders
        (no_wrap, "nowrap", "noWrap", BOOL),    // `<td>` and `<th>`
        (object, "object", "object", STRING),   // `<applet>`
        (profile, "profile", "profile", STRING), // `<head>`
        (prompt, "prompt", "prompt", STRING),   // `<isindex>`
        (rev, "rev", "rev", STRING),            // `<link>`
        (right_margin, "rightmargin", "rightMargin", NUMBER), // `<body>`
        (rules, "rules", "rules", STRING),      // `<table>`
        (scheme, "scheme", "scheme", STRING),   // `<meta>`
        (scrolling, "scrolling", "scrolling", TRUE | FALSE | STRING), // `<frame>`. Use overflow in the child context
        (standby, "standby", "standby", STRING),                      // `<object>`
        (summary, "summary", "summary", STRING),                      // `<table>`
        (text, "text", "text", STRING), // `<body>`. Use CSS `color` instead
        (top_margin, "topmargin", "topMargin", NUMBER), // `<body>`
        (value_type, "valuetype", "valueType", STRING), // `<param>`
        (version, "version", "version", STRING), // `<html>`. Use a doctype.
        (v_align, "valign", "vAlign", STRING), // Several. Use CSS `vertical-align` instead
        (v_link, "vlink", "vLink", STRING), // `<body>`. Use CSS `a:visited {color}` instead
        (v_space, "vspace", "vSpace", NUMBER), // `<img>` and `<object>`
        // Non-standard Properties.
        (
            allow_transparency,
            "allowtransparency",
            "allowTransparency",
            STRING
        ),
        (auto_correct, "autocorrect", "autoCorrect", STRING),
        (auto_save, "autosave", "autoSave", STRING),
        (
            disable_picture_in_picture,
            "disablepictureinpicture",
            "disablePictureInPicture",
            BOOL
        ),
        (
            disable_remote_playback,
            "disableremoteplayback",
            "disableRemotePlayback",
            BOOL
        ),
        (prefix, "prefix", "prefix", STRING),
        (property, "property", "property", STRING),
        (results, "results", "results", NUMBER),
        (security, "security", "security", STRING),
        (unselectable, "unselectable", "unselectable", STRING)
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
