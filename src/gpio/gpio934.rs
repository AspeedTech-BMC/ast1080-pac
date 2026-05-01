#[doc = "Register `GPIO934` reader"]
pub type R = crate::R<Gpio934Spec>;
#[doc = "Register `GPIO934` writer"]
pub type W = crate::W<Gpio934Spec>;
#[doc = "Field `GPIO036ReadPrivilegeOfMaster` reader - GPIO036 Read Privilege of Master"]
pub type Gpio036readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO036ReadPrivilegeOfMaster` writer - GPIO036 Read Privilege of Master"]
pub type Gpio036readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO037ReadPrivilegeOfMaster` reader - GPIO037 Read Privilege of Master"]
pub type Gpio037readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO037ReadPrivilegeOfMaster` writer - GPIO037 Read Privilege of Master"]
pub type Gpio037readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO038ReadPrivilegeOfMaster` reader - GPIO038 Read Privilege of Master"]
pub type Gpio038readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO038ReadPrivilegeOfMaster` writer - GPIO038 Read Privilege of Master"]
pub type Gpio038readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO039ReadPrivilegeOfMaster` reader - GPIO039 Read Privilege of Master"]
pub type Gpio039readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO039ReadPrivilegeOfMaster` writer - GPIO039 Read Privilege of Master"]
pub type Gpio039readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO036 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio036read_privilege_of_master(&self) -> Gpio036readPrivilegeOfMasterR {
        Gpio036readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO037 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio037read_privilege_of_master(&self) -> Gpio037readPrivilegeOfMasterR {
        Gpio037readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO038 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio038read_privilege_of_master(&self) -> Gpio038readPrivilegeOfMasterR {
        Gpio038readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO039 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio039read_privilege_of_master(&self) -> Gpio039readPrivilegeOfMasterR {
        Gpio039readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO036 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio036read_privilege_of_master(
        &mut self,
    ) -> Gpio036readPrivilegeOfMasterW<Gpio934Spec> {
        Gpio036readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO037 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio037read_privilege_of_master(
        &mut self,
    ) -> Gpio037readPrivilegeOfMasterW<Gpio934Spec> {
        Gpio037readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO038 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio038read_privilege_of_master(
        &mut self,
    ) -> Gpio038readPrivilegeOfMasterW<Gpio934Spec> {
        Gpio038readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO039 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio039read_privilege_of_master(
        &mut self,
    ) -> Gpio039readPrivilegeOfMasterW<Gpio934Spec> {
        Gpio039readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio934::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio934::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio934Spec;
impl crate::RegisterSpec for Gpio934Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio934::R`](R) reader structure"]
impl crate::Readable for Gpio934Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio934::W`](W) writer structure"]
impl crate::Writable for Gpio934Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO934 to value 0xffff_ffff"]
impl crate::Resettable for Gpio934Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
