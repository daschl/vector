# 

---
description: Ingests data through kubernetes node's and outputs `log` events.
---

<!--
     THIS FILE IS AUTOGENERATED!

     To make changes please edit the template located at:

     scripts/generate/templates/docs/usage/configuration/sources/kubernetes.md.erb
-->

# kubernetes source

![][assets.kubernetes_source]

{% hint style="warning" %}
The `kubernetes` source is in beta. Please see the current
[enhancements][urls.kubernetes_source_enhancements] and
[bugs][urls.kubernetes_source_bugs] for known issues.
We kindly ask that you [add any missing issues][urls.new_kubernetes_source_issue]
as it will help shape the roadmap of this component.
{% endhint %}

The `kubernetes` source ingests data through kubernetes node's and outputs [`log`][docs.data-model.log] events.

## Example

{% code-tabs %}
{% code-tabs-item title="log" %}
```javascript
{
    "host": "vector-agent-rmqbn",
    "message": "<188>1 2019-10-16T17:50:44.216Z forwardproductize.info quisquam 1693 ID830 - Use the digital ADP capacitor, then you can calculate the solid state capacitor!\n",
    "stream": "stdout",
    "pod_uid": "default_flog-f8dd5f7b-tvgfn_52cdc270-c3e6-4769-b0a9-275481502618",
    "container_name": "flog",
    "timestamp": "2019-10-16T17:51:10.244625907Z"
}
```

{% code-tabs %}
{% code-tabs-item title="vector.toml" %}
```coffeescript
[sources.my_source_id]
  type = "kubernetes" # must be: "kubernetes"
```
{% endcode-tabs-item %}
{% endcode-tabs %}

## Options



## How It Works

### Delivery Guarantee

Due to the nature of this component, it offers a
[**best effort** delivery guarantee][docs.guarantees#best-effort-delivery].

### Deployment

The `kubernetes` source is designed to live on each Kubernetes node as a [`DaemonSet`][urls.kubernetes_daemonset]. You
can find an [example config] in the `vector` repository. At a high level the `kubernetes` source
will run an agent on each node and will internally use the file source to collect logs from `/var/log/pods`
and a few other locations that Kubernetes places logs into. Via the [`DaemonSet`][urls.kubernetes_daemonset] Kubernetes
will ensure that there is always a copy of the agent running on each node.

### Environment Variables

Environment variables are supported through all of Vector's configuration.
Simply add `${MY_ENV_VAR}` in your Vector configuration file and the variable
will be replaced before being evaluated.

You can learn more in the [Environment Variables][docs.configuration#environment-variables]
section.

### High Level

Kubernetes is system to manage containerized based workloads. It internally manages
physical nodes which will then run their designated containers. Vector's Kubernetes
support is designed to run a single instance of the vector agent on each node. This is
done via a [`DaemonSet`][urls.kubernetes_daemonset] which ensures that each node runs one copy of vector on each
node present within the Kubernetes cluster. Each Vector agent will collect logs for all
pods currently deployed running each node.

### Metadata

Each event will contain a `message` field that contains the direct output from the containers stdout/stderr. There
will also be other fields included that come from Kubernetes. These fields include `host`, `stream`, `pod_uid`,
`container_name` and `timestamp`.

## Troubleshooting

The best place to start with troubleshooting is to check the
[Vector logs][docs.monitoring#logs]. This is typically located at
`/var/log/vector.log`, then proceed to follow the
[Troubleshooting Guide][docs.troubleshooting].

If the [Troubleshooting Guide][docs.troubleshooting] does not resolve your
issue, please:

1. Check for any [open `kubernetes_source` issues][urls.kubernetes_source_issues].
2. If encountered a bug, please [file a bug report][urls.new_kubernetes_source_bug].
3. If encountered a missing feature, please [file a feature request][urls.new_kubernetes_source_enhancement].
4. If you need help, [join our chat/forum community][urls.vector_chat]. You can post a question and search previous questions.

## Resources

* [**Issues**][urls.kubernetes_source_issues] - [enhancements][urls.kubernetes_source_enhancements] - [bugs][urls.kubernetes_source_bugs]
* [**Source code**][urls.kubernetes_source_source]


[assets.kubernetes_source]: ../../../assets/kubernetes-source.svg
[docs.configuration#environment-variables]: ../../../usage/configuration#environment-variables
[docs.data-model.log]: ../../../about/data-model/log.md
[docs.guarantees#best-effort-delivery]: ../../../about/guarantees.md#best-effort-delivery
[docs.monitoring#logs]: ../../../usage/administration/monitoring.md#logs
[docs.troubleshooting]: ../../../usage/guides/troubleshooting.md
[urls.kubernetes_daemonset]: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
[urls.kubernetes_source_bugs]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22source%3A+kubernetes%22+label%3A%22Type%3A+bug%22
[urls.kubernetes_source_enhancements]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22source%3A+kubernetes%22+label%3A%22Type%3A+enhancement%22
[urls.kubernetes_source_issues]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22source%3A+kubernetes%22
[urls.kubernetes_source_source]: https://github.com/timberio/vector/tree/master/src/sources/kubernetes.rs
[urls.new_kubernetes_source_bug]: https://github.com/timberio/vector/issues/new?labels=source%3A+kubernetes&labels=Type%3A+bug
[urls.new_kubernetes_source_enhancement]: https://github.com/timberio/vector/issues/new?labels=source%3A+kubernetes&labels=Type%3A+enhancement
[urls.new_kubernetes_source_issue]: https://github.com/timberio/vector/issues/new?labels=source%3A+kubernetes
[urls.vector_chat]: https://chat.vector.dev