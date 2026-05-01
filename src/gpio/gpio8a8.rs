#[doc = "Register `GPIO8A8` reader"]
pub type R = crate::R<Gpio8a8Spec>;
#[doc = "Register `GPIO8A8` writer"]
pub type W = crate::W<Gpio8a8Spec>;
#[doc = "Field `GPIO152WrPrivilegeOfMaster` reader - GPIO152 Write Privilege of Master"]
pub type Gpio152wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO152WrPrivilegeOfMaster` writer - GPIO152 Write Privilege of Master"]
pub type Gpio152wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO153WrPrivilegeOfMaster` reader - GPIO153 Write Privilege of Master"]
pub type Gpio153wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO153WrPrivilegeOfMaster` writer - GPIO153 Write Privilege of Master"]
pub type Gpio153wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO154WrPrivilegeOfMaster` reader - GPIO154 Write Privilege of Master"]
pub type Gpio154wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO154WrPrivilegeOfMaster` writer - GPIO154 Write Privilege of Master"]
pub type Gpio154wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO155WrPrivilegeOfMaster` reader - GPIO155 Write Privilege of Master"]
pub type Gpio155wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO155WrPrivilegeOfMaster` writer - GPIO155 Write Privilege of Master"]
pub type Gpio155wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO152 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio152wr_privilege_of_master(&self) -> Gpio152wrPrivilegeOfMasterR {
        Gpio152wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO153 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio153wr_privilege_of_master(&self) -> Gpio153wrPrivilegeOfMasterR {
        Gpio153wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO154 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio154wr_privilege_of_master(&self) -> Gpio154wrPrivilegeOfMasterR {
        Gpio154wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO155 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio155wr_privilege_of_master(&self) -> Gpio155wrPrivilegeOfMasterR {
        Gpio155wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO152 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio152wr_privilege_of_master(&mut self) -> Gpio152wrPrivilegeOfMasterW<Gpio8a8Spec> {
        Gpio152wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO153 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio153wr_privilege_of_master(&mut self) -> Gpio153wrPrivilegeOfMasterW<Gpio8a8Spec> {
        Gpio153wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO154 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio154wr_privilege_of_master(&mut self) -> Gpio154wrPrivilegeOfMasterW<Gpio8a8Spec> {
        Gpio154wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO155 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio155wr_privilege_of_master(&mut self) -> Gpio155wrPrivilegeOfMasterW<Gpio8a8Spec> {
        Gpio155wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8a8Spec;
impl crate::RegisterSpec for Gpio8a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8a8::R`](R) reader structure"]
impl crate::Readable for Gpio8a8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio8a8::W`](W) writer structure"]
impl crate::Writable for Gpio8a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8A8 to value 0xffff_ffff"]
impl crate::Resettable for Gpio8a8Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
