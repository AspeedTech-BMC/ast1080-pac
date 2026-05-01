#[doc = "Register `GPIO8C8` reader"]
pub type R = crate::R<Gpio8c8Spec>;
#[doc = "Register `GPIO8C8` writer"]
pub type W = crate::W<Gpio8c8Spec>;
#[doc = "Field `GPIO184WrPrivilegeOfMaster` reader - GPIO184 Write Privilege of Master"]
pub type Gpio184wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO184WrPrivilegeOfMaster` writer - GPIO184 Write Privilege of Master"]
pub type Gpio184wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO185WrPrivilegeOfMaster` reader - GPIO185 Write Privilege of Master"]
pub type Gpio185wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO185WrPrivilegeOfMaster` writer - GPIO185 Write Privilege of Master"]
pub type Gpio185wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO186WrPrivilegeOfMaster` reader - GPIO186 Write Privilege of Master"]
pub type Gpio186wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO186WrPrivilegeOfMaster` writer - GPIO186 Write Privilege of Master"]
pub type Gpio186wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO187WrPrivilegeOfMaster` reader - GPIO187 Write Privilege of Master"]
pub type Gpio187wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO187WrPrivilegeOfMaster` writer - GPIO187 Write Privilege of Master"]
pub type Gpio187wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO184 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio184wr_privilege_of_master(&self) -> Gpio184wrPrivilegeOfMasterR {
        Gpio184wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO185 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio185wr_privilege_of_master(&self) -> Gpio185wrPrivilegeOfMasterR {
        Gpio185wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO186 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio186wr_privilege_of_master(&self) -> Gpio186wrPrivilegeOfMasterR {
        Gpio186wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO187 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio187wr_privilege_of_master(&self) -> Gpio187wrPrivilegeOfMasterR {
        Gpio187wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO184 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio184wr_privilege_of_master(&mut self) -> Gpio184wrPrivilegeOfMasterW<Gpio8c8Spec> {
        Gpio184wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO185 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio185wr_privilege_of_master(&mut self) -> Gpio185wrPrivilegeOfMasterW<Gpio8c8Spec> {
        Gpio185wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO186 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio186wr_privilege_of_master(&mut self) -> Gpio186wrPrivilegeOfMasterW<Gpio8c8Spec> {
        Gpio186wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO187 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio187wr_privilege_of_master(&mut self) -> Gpio187wrPrivilegeOfMasterW<Gpio8c8Spec> {
        Gpio187wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#46\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8c8Spec;
impl crate::RegisterSpec for Gpio8c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8c8::R`](R) reader structure"]
impl crate::Readable for Gpio8c8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio8c8::W`](W) writer structure"]
impl crate::Writable for Gpio8c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8C8 to value 0xffff_ffff"]
impl crate::Resettable for Gpio8c8Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
