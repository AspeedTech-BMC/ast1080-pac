#[doc = "Register `GPIO928` reader"]
pub type R = crate::R<Gpio928Spec>;
#[doc = "Register `GPIO928` writer"]
pub type W = crate::W<Gpio928Spec>;
#[doc = "Field `GPIO024ReadPrivilegeOfMaster` reader - GPIO024 Read Privilege of Master"]
pub type Gpio024readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO024ReadPrivilegeOfMaster` writer - GPIO024 Read Privilege of Master"]
pub type Gpio024readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO025ReadPrivilegeOfMaster` reader - GPIO025 Read Privilege of Master"]
pub type Gpio025readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO025ReadPrivilegeOfMaster` writer - GPIO025 Read Privilege of Master"]
pub type Gpio025readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO026ReadPrivilegeOfMaster` reader - GPIO026 Read Privilege of Master"]
pub type Gpio026readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO026ReadPrivilegeOfMaster` writer - GPIO026 Read Privilege of Master"]
pub type Gpio026readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO027ReadPrivilegeOfMaster` reader - GPIO027 Read Privilege of Master"]
pub type Gpio027readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO027ReadPrivilegeOfMaster` writer - GPIO027 Read Privilege of Master"]
pub type Gpio027readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO024 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio024read_privilege_of_master(&self) -> Gpio024readPrivilegeOfMasterR {
        Gpio024readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO025 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio025read_privilege_of_master(&self) -> Gpio025readPrivilegeOfMasterR {
        Gpio025readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO026 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio026read_privilege_of_master(&self) -> Gpio026readPrivilegeOfMasterR {
        Gpio026readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO027 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio027read_privilege_of_master(&self) -> Gpio027readPrivilegeOfMasterR {
        Gpio027readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO024 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio024read_privilege_of_master(
        &mut self,
    ) -> Gpio024readPrivilegeOfMasterW<Gpio928Spec> {
        Gpio024readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO025 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio025read_privilege_of_master(
        &mut self,
    ) -> Gpio025readPrivilegeOfMasterW<Gpio928Spec> {
        Gpio025readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO026 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio026read_privilege_of_master(
        &mut self,
    ) -> Gpio026readPrivilegeOfMasterW<Gpio928Spec> {
        Gpio026readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO027 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio027read_privilege_of_master(
        &mut self,
    ) -> Gpio027readPrivilegeOfMasterW<Gpio928Spec> {
        Gpio027readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio928::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio928::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio928Spec;
impl crate::RegisterSpec for Gpio928Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio928::R`](R) reader structure"]
impl crate::Readable for Gpio928Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio928::W`](W) writer structure"]
impl crate::Writable for Gpio928Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO928 to value 0xffff_ffff"]
impl crate::Resettable for Gpio928Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
