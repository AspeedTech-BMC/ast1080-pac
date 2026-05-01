#[doc = "Register `GPIO988` reader"]
pub type R = crate::R<Gpio988Spec>;
#[doc = "Register `GPIO988` writer"]
pub type W = crate::W<Gpio988Spec>;
#[doc = "Field `GPIO120ReadPrivilegeOfMaster` reader - GPIO120 Read Privilege of Master"]
pub type Gpio120readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO120ReadPrivilegeOfMaster` writer - GPIO120 Read Privilege of Master"]
pub type Gpio120readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO121ReadPrivilegeOfMaster` reader - GPIO121 Read Privilege of Master"]
pub type Gpio121readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO121ReadPrivilegeOfMaster` writer - GPIO121 Read Privilege of Master"]
pub type Gpio121readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO122ReadPrivilegeOfMaster` reader - GPIO122 Read Privilege of Master"]
pub type Gpio122readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO122ReadPrivilegeOfMaster` writer - GPIO122 Read Privilege of Master"]
pub type Gpio122readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO123ReadPrivilegeOfMaster` reader - GPIO123 Read Privilege of Master"]
pub type Gpio123readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO123ReadPrivilegeOfMaster` writer - GPIO123 Read Privilege of Master"]
pub type Gpio123readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO120 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio120read_privilege_of_master(&self) -> Gpio120readPrivilegeOfMasterR {
        Gpio120readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO121 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio121read_privilege_of_master(&self) -> Gpio121readPrivilegeOfMasterR {
        Gpio121readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO122 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio122read_privilege_of_master(&self) -> Gpio122readPrivilegeOfMasterR {
        Gpio122readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO123 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio123read_privilege_of_master(&self) -> Gpio123readPrivilegeOfMasterR {
        Gpio123readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO120 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio120read_privilege_of_master(
        &mut self,
    ) -> Gpio120readPrivilegeOfMasterW<Gpio988Spec> {
        Gpio120readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO121 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio121read_privilege_of_master(
        &mut self,
    ) -> Gpio121readPrivilegeOfMasterW<Gpio988Spec> {
        Gpio121readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO122 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio122read_privilege_of_master(
        &mut self,
    ) -> Gpio122readPrivilegeOfMasterW<Gpio988Spec> {
        Gpio122readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO123 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio123read_privilege_of_master(
        &mut self,
    ) -> Gpio123readPrivilegeOfMasterW<Gpio988Spec> {
        Gpio123readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio988::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio988::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio988Spec;
impl crate::RegisterSpec for Gpio988Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio988::R`](R) reader structure"]
impl crate::Readable for Gpio988Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio988::W`](W) writer structure"]
impl crate::Writable for Gpio988Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO988 to value 0xffff_ffff"]
impl crate::Resettable for Gpio988Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
