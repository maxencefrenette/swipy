import plotly.offline as py
import plotly.graph_objs as go

# Color palette from http://colorbrewer2.org/#type=qualitative&scheme=Dark2&n=8
c = [
    "#1b9e77",
    "#d95f02",
    "#7570b3",
    "#e7298a",
    "#66a61e",
    "#e6ab02",
    "#a6761d",
    "#666666",
]


def plot_many(histories):
    traces = []
    for i, (name, h) in enumerate(histories):
        traces.append(
            go.Scatter(
                x=h["games"],
                y=h["training_scores"],
                name=f"{name} (Training)",
                line={"color": c[i], "dash": "dot"},
            )
        )
        traces.append(
            go.Scatter(
                x=h["games"],
                y=h["test_scores"],
                name=f"{name} (Test)",
                line={"color": c[i]},
            )
        )

    py.plot({"data": traces, "layout": go.Layout(title="Training")}, auto_open=True)
