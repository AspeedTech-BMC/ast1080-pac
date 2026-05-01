#[doc = "Register `GPIO840` reader"]
pub type R = crate::R<Gpio840Spec>;
#[doc = "Register `GPIO840` writer"]
pub type W = crate::W<Gpio840Spec>;
#[doc = "Field `GPIO048WrPrivilegeOfMaster` reader - GPIO048 Write Privilege of Master"]
pub type Gpio048wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO048WrPrivilegeOfMaster` writer - GPIO048 Write Privilege of Master"]
pub type Gpio048wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO049WrPrivilegeOfMaster` reader - GPIO049 Write Privilege of Master"]
pub type Gpio049wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO049WrPrivilegeOfMaster` writer - GPIO049 Write Privilege of Master"]
pub type Gpio049wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO050WrPrivilegeOfMaster` reader - GPIO050 Write Privilege of Master"]
pub type Gpio050wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO050WrPrivilegeOfMaster` writer - GPIO050 Write Privilege of Master"]
pub type Gpio050wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO051WrPrivilegeOfMaster` reader - GPIO051 Write Privilege of Master"]
pub type Gpio051wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO051WrPrivilegeOfMaster` writer - GPIO051 Write Privilege of Master"]
pub type Gpio051wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO048 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio048wr_privilege_of_master(&self) -> Gpio048wrPrivilegeOfMasterR {
        Gpio048wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO049 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio049wr_privilege_of_master(&self) -> Gpio049wrPrivilegeOfMasterR {
        Gpio049wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO050 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio050wr_privilege_of_master(&self) -> Gpio050wrPrivilegeOfMasterR {
        Gpio050wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO051 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio051wr_privilege_of_master(&self) -> Gpio051wrPrivilegeOfMasterR {
        Gpio051wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO048 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio048wr_privilege_of_master(&mut self) -> Gpio048wrPrivilegeOfMasterW<Gpio840Spec> {
        Gpio048wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO049 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio049wr_privilege_of_master(&mut self) -> Gpio049wrPrivilegeOfMasterW<Gpio840Spec> {
        Gpio049wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO050 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio050wr_privilege_of_master(&mut self) -> Gpio050wrPrivilegeOfMasterW<Gpio840Spec> {
        Gpio050wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO051 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio051wr_privilege_of_master(&mut self) -> Gpio051wrPrivilegeOfMasterW<Gpio840Spec> {
        Gpio051wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio840::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio840::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio840Spec;
impl crate::RegisterSpec for Gpio840Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio840::R`](R) reader structure"]
impl crate::Readable for Gpio840Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio840::W`](W) writer structure"]
impl crate::Writable for Gpio840Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO840 to value 0xffff_ffff"]
impl crate::Resettable for Gpio840Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
