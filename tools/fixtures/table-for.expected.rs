rsx! {
    table {
        class: "table table-striped",
        thead {
            tr {
                th {
                    "Name"
                }
                th {
                    "Status"
                }
            }
        }
        tbody {
            for svc in services {
                tr {
                    td {
                        {svc.name}
                    }
                    td {
                        {svc.status}
                    }
                }
            }
        }
    }
}
