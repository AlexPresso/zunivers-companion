import "./WidgetComponent.css"
import {useState} from "react";

export default function WidgetComponent(props: {widget: Widget, current: boolean}) {
    const rgb = hexToRGB(props.widget.color);
    const [ hover, setHover ] = useState(false);

    return (
        <div className={"widget"}
             onMouseEnter={() => setHover(true)}
             onMouseLeave={() => setHover(false)}
             style={{
                 backgroundColor: `rgba(${rgb[0]}, ${rgb[1]}, ${rgb[2]}, 0.3)`,
                 borderColor: props.widget.color,
                 filter: (hover || props.current) ? `drop-shadow(0 0 0.75rem ${props.widget.color})` : "none"
            }}
        >
            <div className={"icon"} style={{backgroundImage: `url(${props.widget.icon})`}} />
            <div className={"infos"}>
                <div className={"name"}>{props.widget.name}</div>
                <div className={"description"}>{props.widget.description}</div>
            </div>
        </div>
    )
}

function hexToRGB(hex: string) {
    return [
        parseInt(hex.slice(1, 3), 16),  //r
        parseInt(hex.slice(3, 5), 16),  //g
        parseInt(hex.slice(5, 7), 16)   //b
    ]
}