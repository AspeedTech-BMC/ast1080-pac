#[doc = "Register `EHCI004` reader"]
pub type R = crate::R<Ehci004Spec>;
#[doc = "Register `EHCI004` writer"]
pub type W = crate::W<Ehci004Spec>;
#[doc = "Field `NPORTS` reader - N_PORTS"]
pub type NportsR = crate::FieldReader;
#[doc = "Field `PortPowerCtrlPPC` reader - Port Power Control (PPC)"]
pub type PortPowerCtrlPpcR = crate::BitReader;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Port Routing Rules\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortRoutingRules {
    #[doc = "0: ] The first N\\_PCC ports are routed to the lowest numbered function"]
    _TheFirstNpccPortsAreRoutedToTheLowestNumberedFunction = 0,
    #[doc = "1: ] The port routing is explicitly enumerated by the first N\\_PORTS elements"]
    _ThePortRoutingIsExplicitlyEnumeratedByTheFirstNportsElements = 1,
}
impl From<PortRoutingRules> for bool {
    #[inline(always)]
    fn from(variant: PortRoutingRules) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PortRoutingRules` reader - Port Routing Rules"]
pub type PortRoutingRulesR = crate::BitReader<PortRoutingRules>;
impl PortRoutingRulesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortRoutingRules {
        match self.bits {
            false => PortRoutingRules::_TheFirstNpccPortsAreRoutedToTheLowestNumberedFunction,
            true => PortRoutingRules::_ThePortRoutingIsExplicitlyEnumeratedByTheFirstNportsElements,
        }
    }
    #[doc = "] The first N\\_PCC ports are routed to the lowest numbered function"]
    #[inline(always)]
    pub fn is_the_first_npcc_ports_are_routed_to_the_lowest_numbered_function(&self) -> bool {
        *self == PortRoutingRules::_TheFirstNpccPortsAreRoutedToTheLowestNumberedFunction
    }
    #[doc = "] The port routing is explicitly enumerated by the first N\\_PORTS elements"]
    #[inline(always)]
    pub fn is_the_port_routing_is_explicitly_enumerated_by_the_first_nports_elements(
        &self,
    ) -> bool {
        *self == PortRoutingRules::_ThePortRoutingIsExplicitlyEnumeratedByTheFirstNportsElements
    }
}
#[doc = "Field `NumberOfPortsPerCompanionCtrlNPCC` reader - Number of Ports per Companion Controller (N_PCC)"]
pub type NumberOfPortsPerCompanionCtrlNpccR = crate::FieldReader;
#[doc = "Field `NumberOfCompanionCtrlNCC` reader - Number of Companion Controller (N_CC)"]
pub type NumberOfCompanionCtrlNccR = crate::FieldReader;
#[doc = "Field `PortIndicatorsPINDICATOR` reader - Port Indicators (P_INDICATOR)"]
pub type PortIndicatorsPindicatorR = crate::BitReader;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `DebugPortNumber` reader - Debug Port Number"]
pub type DebugPortNumberR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - N_PORTS"]
    #[inline(always)]
    pub fn nports(&self) -> NportsR {
        NportsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Port Power Control (PPC)"]
    #[inline(always)]
    pub fn port_power_ctrl_ppc(&self) -> PortPowerCtrlPpcR {
        PortPowerCtrlPpcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Port Routing Rules"]
    #[inline(always)]
    pub fn port_routing_rules(&self) -> PortRoutingRulesR {
        PortRoutingRulesR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Ports per Companion Controller (N_PCC)"]
    #[inline(always)]
    pub fn number_of_ports_per_companion_ctrl_npcc(&self) -> NumberOfPortsPerCompanionCtrlNpccR {
        NumberOfPortsPerCompanionCtrlNpccR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of Companion Controller (N_CC)"]
    #[inline(always)]
    pub fn number_of_companion_ctrl_ncc(&self) -> NumberOfCompanionCtrlNccR {
        NumberOfCompanionCtrlNccR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Port Indicators (P_INDICATOR)"]
    #[inline(always)]
    pub fn port_indicators_pindicator(&self) -> PortIndicatorsPindicatorR {
        PortIndicatorsPindicatorR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Debug Port Number"]
    #[inline(always)]
    pub fn debug_port_number(&self) -> DebugPortNumberR {
        DebugPortNumberR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Structural Parameters (HCSPARAMS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci004Spec;
impl crate::RegisterSpec for Ehci004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci004::R`](R) reader structure"]
impl crate::Readable for Ehci004Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci004::W`](W) writer structure"]
impl crate::Writable for Ehci004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI004 to value 0x1101"]
impl crate::Resettable for Ehci004Spec {
    const RESET_VALUE: u32 = 0x1101;
}
