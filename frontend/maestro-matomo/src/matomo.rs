use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct MatomoProps {
	pub matomo_url: String,
	pub site_id: u32,
}

#[component]
pub fn MatomoScript(props: MatomoProps) -> Element {
	let MatomoProps { matomo_url, site_id } = props;

	let content = format!(
		r#"
      var _paq = (window._paq = window._paq || [])
      /* tracker methods like "setCustomDimension" should be called before "trackPageView" */
      _paq.push(["trackPageView"])
      _paq.push(["enableLinkTracking"])
      ;(function () {{
        var u = "{matomo_url}"
        _paq.push(["setTrackerUrl", u + "matomo.php"])
        _paq.push(["setSiteId", "{site_id}"])
        var d = document, g = d.createElement("script"), s = d.getElementsByTagName("script")[0];
        g.async = true;
        g.src = u + "matomo.js";
        s.parentNode.insertBefore(g, s);
      }})();
    "#,
		matomo_url = matomo_url,
		site_id = site_id,
	);

	rsx! {
		document::Script { dangerous_inner_html: "{content}" }
	}
}
