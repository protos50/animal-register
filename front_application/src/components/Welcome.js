const Welcome = () => {
    return (
        <div style={
            {
                display: "flex",
                justifyContent: "center",
                alignItems: "center",
                flexDirection: "column"
            }
        }>
            <h1>Welcome</h1>
            <img style={{width: "25%", height: "25%"}}
                src="/logo192.png"
                class="img-fluid rounded-top "
                alt=""
            />
            
        </div>
    );
};

export default Welcome;