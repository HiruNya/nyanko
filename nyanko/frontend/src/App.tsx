import React, {} from 'react'
import './App.css'
import { Container, Header, Icon, Menu } from "semantic-ui-react"

function App() {
    return (
        <div className="app">
            <div style={{display: "flex", height: "100%"}}>
                <Menu color="purple" inverted vertical attached style={{maxWidth: "500px", margin: "0", height: "100%"}}>
                    <Menu.Item as="a">
                        <Icon name="home"/>
                        Home
                    </Menu.Item>
                    <Menu.Item as="a" active={true}>
                        <Icon name="search"/>
                        Search
                    </Menu.Item>
                </Menu>
                <Container style={{flex: "auto", display: "grid", placeItems: "center"}}>
                    <Header as="h1">Welcome to Nyanko!</Header>
                </Container>
            </div>
        </div>
    )
}

export default App
