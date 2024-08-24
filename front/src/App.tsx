import {
  Home,
  LifeBuoy,
  Settings,
  Binoculars,
  CirclePlus,
  Table,
  Download,
} from "lucide-react";

import Sidebar, { SidebarItem } from "./components/Sidebar";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Observations from "./Pages/Observations";

function App() {
  return (
    <Router>
      <div className="flex">
        <Sidebar>
          <SidebarItem icon={<Home size={20} />} text="Home" />
          <SidebarItem icon={<Binoculars size={20} />} text="Observations" />
          <SidebarItem
            icon={<CirclePlus size={20} />}
            text="Create observations"
          />
          <SidebarItem icon={<Table size={20} />} text="Manage Tables" />
          <SidebarItem
            icon={<Download size={20} />}
            text="Export observations as CSV"
          />

          <hr className="my-3" />
          <SidebarItem icon={<Settings size={20} />} text="Settings" />
          <SidebarItem icon={<LifeBuoy size={20} />} text="Help" />
        </Sidebar>

        <div className="flex-1 p-4">
          <Routes>
            <Route path="/observations" element={<Observations />} />
            {/* Aquí puedes agregar más rutas en el futuro */}
          </Routes>
        </div>
      </div>
    </Router>
  );
}

export default App;
