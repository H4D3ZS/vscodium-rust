// modern-ssrf-probe.js — APEX Vega modern module (2026)
// Passive response-processor: flags cloud metadata / localhost in response bodies.

var module = {
  name: "Modern SSRF / metadata leak probe",
  type: "response-processor"
};

var MARKERS = [
  "169.254.169.254",
  "metadata.google",
  "100.100.100.200",
  "computeMetadata",
  "ami-id",
  "instance-id",
  "root:x:0:0"
];

function run(request, response, ctx) {
  if (response.fetchFail) return;
  var body = String(response.bodyAsString || "");
  var uri = String(request.requestLine.uri || "");
  for (var i = 0; i < MARKERS.length; i++) {
    if (body.indexOf(MARKERS[i]) >= 0) {
      ctx.alert("vinfo-ssrf-metadata-leak", request, response, {
        output: MARKERS[i],
        resource: uri,
        key: "modern-ssrf:" + uri + ":" + MARKERS[i],
        detectiontype: "Cloud metadata / SSRF indicator in response"
      });
    }
  }
}
