#[doc = "Register `GPIO990` reader"]
pub type R = crate::R<Gpio990Spec>;
#[doc = "Register `GPIO990` writer"]
pub type W = crate::W<Gpio990Spec>;
#[doc = "Field `GPIO128ReadPrivilegeOfMaster` reader - GPIO128 Read Privilege of Master"]
pub type Gpio128readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO128ReadPrivilegeOfMaster` writer - GPIO128 Read Privilege of Master"]
pub type Gpio128readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO129ReadPrivilegeOfMaster` reader - GPIO129 Read Privilege of Master"]
pub type Gpio129readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO129ReadPrivilegeOfMaster` writer - GPIO129 Read Privilege of Master"]
pub type Gpio129readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO130ReadPrivilegeOfMaster` reader - GPIO130 Read Privilege of Master"]
pub type Gpio130readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO130ReadPrivilegeOfMaster` writer - GPIO130 Read Privilege of Master"]
pub type Gpio130readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO131ReadPrivilegeOfMaster` reader - GPIO131 Read Privilege of Master"]
pub type Gpio131readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO131ReadPrivilegeOfMaster` writer - GPIO131 Read Privilege of Master"]
pub type Gpio131readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO128 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio128read_privilege_of_master(&self) -> Gpio128readPrivilegeOfMasterR {
        Gpio128readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO129 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio129read_privilege_of_master(&self) -> Gpio129readPrivilegeOfMasterR {
        Gpio129readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO130 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio130read_privilege_of_master(&self) -> Gpio130readPrivilegeOfMasterR {
        Gpio130readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO131 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio131read_privilege_of_master(&self) -> Gpio131readPrivilegeOfMasterR {
        Gpio131readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO128 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio128read_privilege_of_master(
        &mut self,
    ) -> Gpio128readPrivilegeOfMasterW<Gpio990Spec> {
        Gpio128readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO129 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio129read_privilege_of_master(
        &mut self,
    ) -> Gpio129readPrivilegeOfMasterW<Gpio990Spec> {
        Gpio129readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO130 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio130read_privilege_of_master(
        &mut self,
    ) -> Gpio130readPrivilegeOfMasterW<Gpio990Spec> {
        Gpio130readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO131 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio131read_privilege_of_master(
        &mut self,
    ) -> Gpio131readPrivilegeOfMasterW<Gpio990Spec> {
        Gpio131readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio990::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio990::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio990Spec;
impl crate::RegisterSpec for Gpio990Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio990::R`](R) reader structure"]
impl crate::Readable for Gpio990Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio990::W`](W) writer structure"]
impl crate::Writable for Gpio990Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO990 to value 0xffff_ffff"]
impl crate::Resettable for Gpio990Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
