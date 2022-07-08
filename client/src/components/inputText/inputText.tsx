import { ChangeEventHandler } from "react";

interface InputTextParams {
    onChange?: ChangeEventHandler<HTMLInputElement>;
    // TODO: add customibilty, aka colors, size, whatnot
}

const InputText = ({ onChange }: InputTextParams) => {
    return (
        <>
            <input type="text" onChange={onChange} />
        </>
    );
};

export default InputText;
