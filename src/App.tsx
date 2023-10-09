import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import JsxParser from "react-jsx-parser";
import WidgetComponent from "./components/WidgetComponent.tsx";

function App() {
    const [ widgets, setWidgets ] = useState<Widget[]>([])
    const [ currWidgetIdx, setCurrWidgetIdx ] = useState(0);
    const [ currRender, setCurrRender ] = useState("");

    useEffect(() => {
        loadWidgets();
    }, []);

    useEffect(() => {
        renderWidget();
    }, [widgets, currWidgetIdx])

    async function loadWidgets() {
        const widgets: Widget[] = await invoke("get_widgets");
        setWidgets(widgets);
    }

    async function renderWidget() {
        const currentWidget = widgets[currWidgetIdx];
        let render = "";

        if(currentWidget)
            render = await invoke("render_widget", { widgetName: currentWidget.name });

        setCurrRender(render);
    }

    return (
        <div className="container">
            <nav className={"left-menu"}>
                <ul>
                    {widgets.map((widget, i) =>
                        <li onClick={() => setCurrWidgetIdx(i)} key={i}>
                            <WidgetComponent widget={widget} current={i === currWidgetIdx} />
                        </li>
                    )}
                </ul>
            </nav>
            <main className={"content"}>
                {currRender ?
                    // @ts-ignore
                    <JsxParser
                        autoCloseVoidElements={true}
                        jsx={currRender}
                    /> : <></>
                }
            </main>
        </div>
    );
}

export default App;
