#[doc = "Register `GPIO8B0` reader"]
pub type R = crate::R<Gpio8b0Spec>;
#[doc = "Register `GPIO8B0` writer"]
pub type W = crate::W<Gpio8b0Spec>;
#[doc = "Field `GPIO160WrPrivilegeOfMaster` reader - GPIO160 Write Privilege of Master"]
pub type Gpio160wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO160WrPrivilegeOfMaster` writer - GPIO160 Write Privilege of Master"]
pub type Gpio160wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO161WrPrivilegeOfMaster` reader - GPIO161 Write Privilege of Master"]
pub type Gpio161wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO161WrPrivilegeOfMaster` writer - GPIO161 Write Privilege of Master"]
pub type Gpio161wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO162WrPrivilegeOfMaster` reader - GPIO162 Write Privilege of Master"]
pub type Gpio162wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO162WrPrivilegeOfMaster` writer - GPIO162 Write Privilege of Master"]
pub type Gpio162wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO163WrPrivilegeOfMaster` reader - GPIO163 Write Privilege of Master"]
pub type Gpio163wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO163WrPrivilegeOfMaster` writer - GPIO163 Write Privilege of Master"]
pub type Gpio163wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO160 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio160wr_privilege_of_master(&self) -> Gpio160wrPrivilegeOfMasterR {
        Gpio160wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO161 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio161wr_privilege_of_master(&self) -> Gpio161wrPrivilegeOfMasterR {
        Gpio161wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO162 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio162wr_privilege_of_master(&self) -> Gpio162wrPrivilegeOfMasterR {
        Gpio162wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO163 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio163wr_privilege_of_master(&self) -> Gpio163wrPrivilegeOfMasterR {
        Gpio163wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO160 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio160wr_privilege_of_master(&mut self) -> Gpio160wrPrivilegeOfMasterW<Gpio8b0Spec> {
        Gpio160wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO161 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio161wr_privilege_of_master(&mut self) -> Gpio161wrPrivilegeOfMasterW<Gpio8b0Spec> {
        Gpio161wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO162 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio162wr_privilege_of_master(&mut self) -> Gpio162wrPrivilegeOfMasterW<Gpio8b0Spec> {
        Gpio162wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO163 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio163wr_privilege_of_master(&mut self) -> Gpio163wrPrivilegeOfMasterW<Gpio8b0Spec> {
        Gpio163wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8b0Spec;
impl crate::RegisterSpec for Gpio8b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8b0::R`](R) reader structure"]
impl crate::Readable for Gpio8b0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio8b0::W`](W) writer structure"]
impl crate::Writable for Gpio8b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8B0 to value 0xffff_ffff"]
impl crate::Resettable for Gpio8b0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
