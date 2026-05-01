#[doc = "Register `EHCI060` reader"]
pub type R = crate::R<Ehci060Spec>;
#[doc = "Register `EHCI060` writer"]
pub type W = crate::W<Ehci060Spec>;
#[doc = "Configure Flag (CF)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigureFlagCf {
    #[doc = "0: ] Port routing control logic default-routes each port to an implementation"]
    _PortRoutingControlLogicDefaultroutesEachPortToAnImplementation = 0,
    #[doc = "1: ] Port routing control logic default-routes all ports to this host controller."]
    _PortRoutingControlLogicDefaultroutesAllPortsToThisHostController = 1,
}
impl From<ConfigureFlagCf> for bool {
    #[inline(always)]
    fn from(variant: ConfigureFlagCf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ConfigureFlagCF` reader - Configure Flag (CF)"]
pub type ConfigureFlagCfR = crate::BitReader<ConfigureFlagCf>;
impl ConfigureFlagCfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ConfigureFlagCf {
        match self.bits {
            false => {
                ConfigureFlagCf::_PortRoutingControlLogicDefaultroutesEachPortToAnImplementation
            }
            true => {
                ConfigureFlagCf::_PortRoutingControlLogicDefaultroutesAllPortsToThisHostController
            }
        }
    }
    #[doc = "] Port routing control logic default-routes each port to an implementation"]
    #[inline(always)]
    pub fn is_port_routing_control_logic_defaultroutes_each_port_to_an_implementation(
        &self,
    ) -> bool {
        *self == ConfigureFlagCf::_PortRoutingControlLogicDefaultroutesEachPortToAnImplementation
    }
    #[doc = "] Port routing control logic default-routes all ports to this host controller."]
    #[inline(always)]
    pub fn is_port_routing_control_logic_defaultroutes_all_ports_to_this_host_controller(
        &self,
    ) -> bool {
        *self == ConfigureFlagCf::_PortRoutingControlLogicDefaultroutesAllPortsToThisHostController
    }
}
#[doc = "Field `ConfigureFlagCF` writer - Configure Flag (CF)"]
pub type ConfigureFlagCfW<'a, REG> = crate::BitWriter<'a, REG, ConfigureFlagCf>;
impl<'a, REG> ConfigureFlagCfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "] Port routing control logic default-routes each port to an implementation"]
    #[inline(always)]
    pub fn _port_routing_control_logic_defaultroutes_each_port_to_an_implementation(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(
            ConfigureFlagCf::_PortRoutingControlLogicDefaultroutesEachPortToAnImplementation,
        )
    }
    #[doc = "] Port routing control logic default-routes all ports to this host controller."]
    #[inline(always)]
    pub fn _port_routing_control_logic_defaultroutes_all_ports_to_this_host_controller(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(
            ConfigureFlagCf::_PortRoutingControlLogicDefaultroutesAllPortsToThisHostController,
        )
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Configure Flag (CF)"]
    #[inline(always)]
    pub fn configure_flag_cf(&self) -> ConfigureFlagCfR {
        ConfigureFlagCfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Configure Flag (CF)"]
    #[inline(always)]
    pub fn configure_flag_cf(&mut self) -> ConfigureFlagCfW<Ehci060Spec> {
        ConfigureFlagCfW::new(self, 0)
    }
}
#[doc = "Configure Flag Register (CONFIGFLAG)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci060Spec;
impl crate::RegisterSpec for Ehci060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci060::R`](R) reader structure"]
impl crate::Readable for Ehci060Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci060::W`](W) writer structure"]
impl crate::Writable for Ehci060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI060 to value 0"]
impl crate::Resettable for Ehci060Spec {}
