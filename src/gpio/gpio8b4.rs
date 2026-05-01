#[doc = "Register `GPIO8B4` reader"]
pub type R = crate::R<Gpio8b4Spec>;
#[doc = "Register `GPIO8B4` writer"]
pub type W = crate::W<Gpio8b4Spec>;
#[doc = "Field `GPIO164WrPrivilegeOfMaster` reader - GPIO164 Write Privilege of Master"]
pub type Gpio164wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO164WrPrivilegeOfMaster` writer - GPIO164 Write Privilege of Master"]
pub type Gpio164wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO165WrPrivilegeOfMaster` reader - GPIO165 Write Privilege of Master"]
pub type Gpio165wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO165WrPrivilegeOfMaster` writer - GPIO165 Write Privilege of Master"]
pub type Gpio165wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO166WrPrivilegeOfMaster` reader - GPIO166 Write Privilege of Master"]
pub type Gpio166wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO166WrPrivilegeOfMaster` writer - GPIO166 Write Privilege of Master"]
pub type Gpio166wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO167WrPrivilegeOfMaster` reader - GPIO167 Write Privilege of Master"]
pub type Gpio167wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO167WrPrivilegeOfMaster` writer - GPIO167 Write Privilege of Master"]
pub type Gpio167wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO164 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio164wr_privilege_of_master(&self) -> Gpio164wrPrivilegeOfMasterR {
        Gpio164wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO165 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio165wr_privilege_of_master(&self) -> Gpio165wrPrivilegeOfMasterR {
        Gpio165wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO166 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio166wr_privilege_of_master(&self) -> Gpio166wrPrivilegeOfMasterR {
        Gpio166wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO167 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio167wr_privilege_of_master(&self) -> Gpio167wrPrivilegeOfMasterR {
        Gpio167wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO164 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio164wr_privilege_of_master(&mut self) -> Gpio164wrPrivilegeOfMasterW<Gpio8b4Spec> {
        Gpio164wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO165 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio165wr_privilege_of_master(&mut self) -> Gpio165wrPrivilegeOfMasterW<Gpio8b4Spec> {
        Gpio165wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO166 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio166wr_privilege_of_master(&mut self) -> Gpio166wrPrivilegeOfMasterW<Gpio8b4Spec> {
        Gpio166wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO167 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio167wr_privilege_of_master(&mut self) -> Gpio167wrPrivilegeOfMasterW<Gpio8b4Spec> {
        Gpio167wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8b4Spec;
impl crate::RegisterSpec for Gpio8b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8b4::R`](R) reader structure"]
impl crate::Readable for Gpio8b4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio8b4::W`](W) writer structure"]
impl crate::Writable for Gpio8b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8B4 to value 0xffff_ffff"]
impl crate::Resettable for Gpio8b4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
