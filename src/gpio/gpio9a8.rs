#[doc = "Register `GPIO9A8` reader"]
pub type R = crate::R<Gpio9a8Spec>;
#[doc = "Register `GPIO9A8` writer"]
pub type W = crate::W<Gpio9a8Spec>;
#[doc = "Field `GPIO152ReadPrivilegeOfMaster` reader - GPIO152 Read Privilege of Master"]
pub type Gpio152readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO152ReadPrivilegeOfMaster` writer - GPIO152 Read Privilege of Master"]
pub type Gpio152readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO153ReadPrivilegeOfMaster` reader - GPIO153 Read Privilege of Master"]
pub type Gpio153readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO153ReadPrivilegeOfMaster` writer - GPIO153 Read Privilege of Master"]
pub type Gpio153readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO154ReadPrivilegeOfMaster` reader - GPIO154 Read Privilege of Master"]
pub type Gpio154readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO154ReadPrivilegeOfMaster` writer - GPIO154 Read Privilege of Master"]
pub type Gpio154readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO155ReadPrivilegeOfMaster` reader - GPIO155 Read Privilege of Master"]
pub type Gpio155readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO155ReadPrivilegeOfMaster` writer - GPIO155 Read Privilege of Master"]
pub type Gpio155readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO152 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio152read_privilege_of_master(&self) -> Gpio152readPrivilegeOfMasterR {
        Gpio152readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO153 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio153read_privilege_of_master(&self) -> Gpio153readPrivilegeOfMasterR {
        Gpio153readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO154 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio154read_privilege_of_master(&self) -> Gpio154readPrivilegeOfMasterR {
        Gpio154readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO155 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio155read_privilege_of_master(&self) -> Gpio155readPrivilegeOfMasterR {
        Gpio155readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO152 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio152read_privilege_of_master(
        &mut self,
    ) -> Gpio152readPrivilegeOfMasterW<Gpio9a8Spec> {
        Gpio152readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO153 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio153read_privilege_of_master(
        &mut self,
    ) -> Gpio153readPrivilegeOfMasterW<Gpio9a8Spec> {
        Gpio153readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO154 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio154read_privilege_of_master(
        &mut self,
    ) -> Gpio154readPrivilegeOfMasterW<Gpio9a8Spec> {
        Gpio154readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO155 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio155read_privilege_of_master(
        &mut self,
    ) -> Gpio155readPrivilegeOfMasterW<Gpio9a8Spec> {
        Gpio155readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9a8Spec;
impl crate::RegisterSpec for Gpio9a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9a8::R`](R) reader structure"]
impl crate::Readable for Gpio9a8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio9a8::W`](W) writer structure"]
impl crate::Writable for Gpio9a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9A8 to value 0xffff_ffff"]
impl crate::Resettable for Gpio9a8Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
