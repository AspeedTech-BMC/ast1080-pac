#[doc = "Register `GPIO960` reader"]
pub type R = crate::R<Gpio960Spec>;
#[doc = "Register `GPIO960` writer"]
pub type W = crate::W<Gpio960Spec>;
#[doc = "Field `GPIO080ReadPrivilegeOfMaster` reader - GPIO080 Read Privilege of Master"]
pub type Gpio080readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO080ReadPrivilegeOfMaster` writer - GPIO080 Read Privilege of Master"]
pub type Gpio080readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO081ReadPrivilegeOfMaster` reader - GPIO081 Read Privilege of Master"]
pub type Gpio081readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO081ReadPrivilegeOfMaster` writer - GPIO081 Read Privilege of Master"]
pub type Gpio081readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO082ReadPrivilegeOfMaster` reader - GPIO082 Read Privilege of Master"]
pub type Gpio082readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO082ReadPrivilegeOfMaster` writer - GPIO082 Read Privilege of Master"]
pub type Gpio082readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO083ReadPrivilegeOfMaster` reader - GPIO083 Read Privilege of Master"]
pub type Gpio083readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO083ReadPrivilegeOfMaster` writer - GPIO083 Read Privilege of Master"]
pub type Gpio083readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO080 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio080read_privilege_of_master(&self) -> Gpio080readPrivilegeOfMasterR {
        Gpio080readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO081 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio081read_privilege_of_master(&self) -> Gpio081readPrivilegeOfMasterR {
        Gpio081readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO082 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio082read_privilege_of_master(&self) -> Gpio082readPrivilegeOfMasterR {
        Gpio082readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO083 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio083read_privilege_of_master(&self) -> Gpio083readPrivilegeOfMasterR {
        Gpio083readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO080 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio080read_privilege_of_master(
        &mut self,
    ) -> Gpio080readPrivilegeOfMasterW<Gpio960Spec> {
        Gpio080readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO081 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio081read_privilege_of_master(
        &mut self,
    ) -> Gpio081readPrivilegeOfMasterW<Gpio960Spec> {
        Gpio081readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO082 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio082read_privilege_of_master(
        &mut self,
    ) -> Gpio082readPrivilegeOfMasterW<Gpio960Spec> {
        Gpio082readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO083 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio083read_privilege_of_master(
        &mut self,
    ) -> Gpio083readPrivilegeOfMasterW<Gpio960Spec> {
        Gpio083readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio960::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio960::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio960Spec;
impl crate::RegisterSpec for Gpio960Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio960::R`](R) reader structure"]
impl crate::Readable for Gpio960Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio960::W`](W) writer structure"]
impl crate::Writable for Gpio960Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO960 to value 0xffff_ffff"]
impl crate::Resettable for Gpio960Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
