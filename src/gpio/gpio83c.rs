#[doc = "Register `GPIO83C` reader"]
pub type R = crate::R<Gpio83cSpec>;
#[doc = "Register `GPIO83C` writer"]
pub type W = crate::W<Gpio83cSpec>;
#[doc = "Field `GPIO044WrPrivilegeOfMaster` reader - GPIO044 Write Privilege of Master"]
pub type Gpio044wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO044WrPrivilegeOfMaster` writer - GPIO044 Write Privilege of Master"]
pub type Gpio044wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO045WrPrivilegeOfMaster` reader - GPIO045 Write Privilege of Master"]
pub type Gpio045wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO045WrPrivilegeOfMaster` writer - GPIO045 Write Privilege of Master"]
pub type Gpio045wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO046WrPrivilegeOfMaster` reader - GPIO046 Write Privilege of Master"]
pub type Gpio046wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO046WrPrivilegeOfMaster` writer - GPIO046 Write Privilege of Master"]
pub type Gpio046wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO047WrPrivilegeOfMaster` reader - GPIO047 Write Privilege of Master"]
pub type Gpio047wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO047WrPrivilegeOfMaster` writer - GPIO047 Write Privilege of Master"]
pub type Gpio047wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO044 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio044wr_privilege_of_master(&self) -> Gpio044wrPrivilegeOfMasterR {
        Gpio044wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO045 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio045wr_privilege_of_master(&self) -> Gpio045wrPrivilegeOfMasterR {
        Gpio045wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO046 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio046wr_privilege_of_master(&self) -> Gpio046wrPrivilegeOfMasterR {
        Gpio046wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO047 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio047wr_privilege_of_master(&self) -> Gpio047wrPrivilegeOfMasterR {
        Gpio047wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO044 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio044wr_privilege_of_master(&mut self) -> Gpio044wrPrivilegeOfMasterW<Gpio83cSpec> {
        Gpio044wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO045 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio045wr_privilege_of_master(&mut self) -> Gpio045wrPrivilegeOfMasterW<Gpio83cSpec> {
        Gpio045wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO046 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio046wr_privilege_of_master(&mut self) -> Gpio046wrPrivilegeOfMasterW<Gpio83cSpec> {
        Gpio046wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO047 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio047wr_privilege_of_master(&mut self) -> Gpio047wrPrivilegeOfMasterW<Gpio83cSpec> {
        Gpio047wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio83c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio83c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio83cSpec;
impl crate::RegisterSpec for Gpio83cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio83c::R`](R) reader structure"]
impl crate::Readable for Gpio83cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio83c::W`](W) writer structure"]
impl crate::Writable for Gpio83cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO83C to value 0xffff_ffff"]
impl crate::Resettable for Gpio83cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
