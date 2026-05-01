#[doc = "Register `GPIO93C` reader"]
pub type R = crate::R<Gpio93cSpec>;
#[doc = "Register `GPIO93C` writer"]
pub type W = crate::W<Gpio93cSpec>;
#[doc = "Field `GPIO044ReadPrivilegeOfMaster` reader - GPIO044 Read Privilege of Master"]
pub type Gpio044readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO044ReadPrivilegeOfMaster` writer - GPIO044 Read Privilege of Master"]
pub type Gpio044readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO045ReadPrivilegeOfMaster` reader - GPIO045 Read Privilege of Master"]
pub type Gpio045readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO045ReadPrivilegeOfMaster` writer - GPIO045 Read Privilege of Master"]
pub type Gpio045readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO046ReadPrivilegeOfMaster` reader - GPIO046 Read Privilege of Master"]
pub type Gpio046readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO046ReadPrivilegeOfMaster` writer - GPIO046 Read Privilege of Master"]
pub type Gpio046readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO047ReadPrivilegeOfMaster` reader - GPIO047 Read Privilege of Master"]
pub type Gpio047readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO047ReadPrivilegeOfMaster` writer - GPIO047 Read Privilege of Master"]
pub type Gpio047readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO044 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio044read_privilege_of_master(&self) -> Gpio044readPrivilegeOfMasterR {
        Gpio044readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO045 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio045read_privilege_of_master(&self) -> Gpio045readPrivilegeOfMasterR {
        Gpio045readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO046 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio046read_privilege_of_master(&self) -> Gpio046readPrivilegeOfMasterR {
        Gpio046readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO047 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio047read_privilege_of_master(&self) -> Gpio047readPrivilegeOfMasterR {
        Gpio047readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO044 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio044read_privilege_of_master(
        &mut self,
    ) -> Gpio044readPrivilegeOfMasterW<Gpio93cSpec> {
        Gpio044readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO045 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio045read_privilege_of_master(
        &mut self,
    ) -> Gpio045readPrivilegeOfMasterW<Gpio93cSpec> {
        Gpio045readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO046 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio046read_privilege_of_master(
        &mut self,
    ) -> Gpio046readPrivilegeOfMasterW<Gpio93cSpec> {
        Gpio046readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO047 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio047read_privilege_of_master(
        &mut self,
    ) -> Gpio047readPrivilegeOfMasterW<Gpio93cSpec> {
        Gpio047readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio93c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio93c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio93cSpec;
impl crate::RegisterSpec for Gpio93cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio93c::R`](R) reader structure"]
impl crate::Readable for Gpio93cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio93c::W`](W) writer structure"]
impl crate::Writable for Gpio93cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO93C to value 0xffff_ffff"]
impl crate::Resettable for Gpio93cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
