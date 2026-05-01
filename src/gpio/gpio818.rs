#[doc = "Register `GPIO818` reader"]
pub type R = crate::R<Gpio818Spec>;
#[doc = "Register `GPIO818` writer"]
pub type W = crate::W<Gpio818Spec>;
#[doc = "Field `GPIO008WrPrivilegeOfMaster` reader - GPIO008 Write Privilege of Master"]
pub type Gpio008wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO008WrPrivilegeOfMaster` writer - GPIO008 Write Privilege of Master"]
pub type Gpio008wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO009WrPrivilegeOfMaster` reader - GPIO009 Write Privilege of Master"]
pub type Gpio009wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO009WrPrivilegeOfMaster` writer - GPIO009 Write Privilege of Master"]
pub type Gpio009wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO010WrPrivilegeOfMaster` reader - GPIO010 Write Privilege of Master"]
pub type Gpio010wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO010WrPrivilegeOfMaster` writer - GPIO010 Write Privilege of Master"]
pub type Gpio010wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO011WrPrivilegeOfMaster` reader - GPIO011 Write Privilege of Master"]
pub type Gpio011wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO011WrPrivilegeOfMaster` writer - GPIO011 Write Privilege of Master"]
pub type Gpio011wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO008 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio008wr_privilege_of_master(&self) -> Gpio008wrPrivilegeOfMasterR {
        Gpio008wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO009 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio009wr_privilege_of_master(&self) -> Gpio009wrPrivilegeOfMasterR {
        Gpio009wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO010 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio010wr_privilege_of_master(&self) -> Gpio010wrPrivilegeOfMasterR {
        Gpio010wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO011 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio011wr_privilege_of_master(&self) -> Gpio011wrPrivilegeOfMasterR {
        Gpio011wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO008 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio008wr_privilege_of_master(&mut self) -> Gpio008wrPrivilegeOfMasterW<Gpio818Spec> {
        Gpio008wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO009 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio009wr_privilege_of_master(&mut self) -> Gpio009wrPrivilegeOfMasterW<Gpio818Spec> {
        Gpio009wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO010 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio010wr_privilege_of_master(&mut self) -> Gpio010wrPrivilegeOfMasterW<Gpio818Spec> {
        Gpio010wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO011 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio011wr_privilege_of_master(&mut self) -> Gpio011wrPrivilegeOfMasterW<Gpio818Spec> {
        Gpio011wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio818::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio818::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio818Spec;
impl crate::RegisterSpec for Gpio818Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio818::R`](R) reader structure"]
impl crate::Readable for Gpio818Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio818::W`](W) writer structure"]
impl crate::Writable for Gpio818Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO818 to value 0xffff_ffff"]
impl crate::Resettable for Gpio818Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
