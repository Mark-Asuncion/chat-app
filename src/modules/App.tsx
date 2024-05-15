import { useEffect, useState } from 'react';
import { useNavigate } from "react-router-dom";

function App() {
    const [isLogin, _setIsLogin] = useState(false);
    const navigate = useNavigate();
    useEffect(() => {
        console.log("useEffect");
        if (isLogin) {
            navigate("/");
        }
        else {
            navigate("/login");
        }
    });
    return (
        <p className='text-6xl text-white'>Home</p>
    );
}

export default App;
