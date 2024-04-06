import { ChangeEventHandler } from "react";
import "./../styles/slider.css";

const ModeSwitcher = (props: {
  onChange: ChangeEventHandler<HTMLInputElement> | undefined;
  mode: string;
}) => {
  return (
    <div className="float-right mt-3 w-full p-4 text-lg">
      <label className="switch justify-content align-middle">
        <input type="checkbox" onChange={props.onChange} />
        <span className="slider round"></span>
      </label>
      <span className="px-2">{props.mode}</span>
    </div>
  );
};

export default ModeSwitcher;
