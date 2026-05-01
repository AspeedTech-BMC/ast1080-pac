#[doc = "Register `GPIO930` reader"]
pub type R = crate::R<Gpio930Spec>;
#[doc = "Register `GPIO930` writer"]
pub type W = crate::W<Gpio930Spec>;
#[doc = "Field `GPIO032ReadPrivilegeOfMaster` reader - GPIO032 Read Privilege of Master"]
pub type Gpio032readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO032ReadPrivilegeOfMaster` writer - GPIO032 Read Privilege of Master"]
pub type Gpio032readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO033ReadPrivilegeOfMaster` reader - GPIO033 Read Privilege of Master"]
pub type Gpio033readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO033ReadPrivilegeOfMaster` writer - GPIO033 Read Privilege of Master"]
pub type Gpio033readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO034ReadPrivilegeOfMaster` reader - GPIO034 Read Privilege of Master"]
pub type Gpio034readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO034ReadPrivilegeOfMaster` writer - GPIO034 Read Privilege of Master"]
pub type Gpio034readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO035ReadPrivilegeOfMaster` reader - GPIO035 Read Privilege of Master"]
pub type Gpio035readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO035ReadPrivilegeOfMaster` writer - GPIO035 Read Privilege of Master"]
pub type Gpio035readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO032 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio032read_privilege_of_master(&self) -> Gpio032readPrivilegeOfMasterR {
        Gpio032readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO033 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio033read_privilege_of_master(&self) -> Gpio033readPrivilegeOfMasterR {
        Gpio033readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO034 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio034read_privilege_of_master(&self) -> Gpio034readPrivilegeOfMasterR {
        Gpio034readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO035 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio035read_privilege_of_master(&self) -> Gpio035readPrivilegeOfMasterR {
        Gpio035readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO032 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio032read_privilege_of_master(
        &mut self,
    ) -> Gpio032readPrivilegeOfMasterW<Gpio930Spec> {
        Gpio032readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO033 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio033read_privilege_of_master(
        &mut self,
    ) -> Gpio033readPrivilegeOfMasterW<Gpio930Spec> {
        Gpio033readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO034 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio034read_privilege_of_master(
        &mut self,
    ) -> Gpio034readPrivilegeOfMasterW<Gpio930Spec> {
        Gpio034readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO035 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio035read_privilege_of_master(
        &mut self,
    ) -> Gpio035readPrivilegeOfMasterW<Gpio930Spec> {
        Gpio035readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio930::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio930::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio930Spec;
impl crate::RegisterSpec for Gpio930Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio930::R`](R) reader structure"]
impl crate::Readable for Gpio930Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio930::W`](W) writer structure"]
impl crate::Writable for Gpio930Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO930 to value 0xffff_ffff"]
impl crate::Resettable for Gpio930Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
